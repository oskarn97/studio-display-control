#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{arg, Command};
use hidapi::{self, HidApi};
use log::*;
use std::{error::Error, vec::Vec};

const REPORT_ID: u8 = 1;
const MIN_BRIGHTNESS: u32 = 400;
const MAX_BRIGHTNESS: u32 = 60000;
const BRIGHTNESS_RANGE: u32 = MAX_BRIGHTNESS - MIN_BRIGHTNESS;

const SD_PRODUCT_ID: u16 = 0x1114;
const SD_VENDOR_ID: u16 = 0x05ac;
const SD_INTERFACE_NR: i32 = 0x7;

fn get_brightness(handle: &mut hidapi::HidDevice) -> Result<u32, Box<dyn Error>> {
    let mut buf = Vec::with_capacity(7);
    buf.push(REPORT_ID);
    buf.extend(0_u32.to_le_bytes());
    buf.extend(0_u16.to_le_bytes());
    let size = handle.get_feature_report(&mut buf)?;
    if size != buf.len() {
        Err(format!(
            "Get HID feature report: Expected a size of {}, got {}",
            buf.len(),
            size
        ))?
    }
    let brightness = u32::from_le_bytes(buf[1..5].try_into()?);
    Ok(brightness)
}

fn get_brightness_percent(handle: &mut hidapi::HidDevice) -> Result<u8, Box<dyn Error>> {
    let value = (get_brightness(handle)? - MIN_BRIGHTNESS) as f32;
    let value_percent = (value / BRIGHTNESS_RANGE as f32 * 100.0) as u8;
    Ok(value_percent)
}

fn set_brightness(handle: &mut hidapi::HidDevice, brightness: u32) -> Result<(), Box<dyn Error>> {
    let mut buf = Vec::with_capacity(7);
    buf.push(REPORT_ID);
    buf.extend(brightness.to_le_bytes());
    buf.extend(0_u16.to_le_bytes());
    handle.send_feature_report(&mut buf)?;
    Ok(())
}

fn set_brightness_percent(handle: &mut hidapi::HidDevice, brightness: u8) -> Result<(), Box<dyn Error>> {
    let nits =
        ((brightness as f32 * BRIGHTNESS_RANGE as f32) / 100.0 + MIN_BRIGHTNESS as f32) as u32;
    let nits = std::cmp::min(nits, MAX_BRIGHTNESS);
    let nits = std::cmp::max(nits, MIN_BRIGHTNESS);
    set_brightness(handle, nits)?;
    Ok(())
}

fn studio_displays(hapi: &HidApi) -> Result<Vec<&hidapi::DeviceInfo>, Box<dyn Error>> {
    Ok(hapi
        .device_list()
        .filter(|x| {
            x.product_id() == SD_PRODUCT_ID
                && x.vendor_id() == SD_VENDOR_ID
                && x.interface_number() == SD_INTERFACE_NR
        })
        .collect())
}

fn cli() -> Command {
    Command::new("asdbctl")
        .about("Tool to get or set the brightness for Apple Studio Displays. Launches UI if no command is given.")
        
        .arg(arg!(-s --serial <SERIAL> "Serial number of the display for which to adjust the brightness"))
        .arg(arg!(-v --verbose ... "Turn debugging information on"))
        .subcommand(Command::new("get").about("Get the current brightness in %"))
        .subcommand(
            Command::new("set")
                .about("Set the current brightness in %")
                .arg(
                    arg!(<BRIGHTNESS> "Brightness percentage")
                        .value_parser(clap::value_parser!(u8).range(0..101)),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("up")
                .arg(
                    arg!(-s --step <STEP> "Step size in percent")
                        .required(false)
                        .default_value("10")
                        .value_parser(clap::value_parser!(u8).range(1..101)),
                )
                .about("Increase the brightness"),
        )
        .subcommand(
            Command::new("down")
                .arg(
                    arg!(-s --step <STEP> "Step size in percent")
                        .required(false)
                        .default_value("10")
                        .value_parser(clap::value_parser!(u8).range(1..101)),
                )
                .about("Decrease the brightness"),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli().get_matches();
    let verbosity = *matches.get_one::<u8>("verbose").unwrap_or(&0) as usize;
    stderrlog::new().module(module_path!()).verbosity(verbosity).init().unwrap();

    // If no subcommand is provided, launch the GUI.
    if matches.subcommand().is_none() {
        let serial = matches.get_one::<String>("serial").map(|s| s.to_string());
        gui::launch_gui(serial)?;
        return Ok(());
    }

    // --- Existing CLI mode ---
    let hapi = HidApi::new()?;
    let displays = studio_displays(&hapi)?;
    if displays.is_empty() {
        Err("No Apple Studio Display found")?;
    }

    for display in displays {
        let mut handle = hapi.open_path(display.path())?;
        if let Some(s) = display.serial_number() {
            info!("display serial number {}", s);
        }
        if let Some(serial) = matches.get_one::<String>("serial") {
            if let Some(s) = display.serial_number() {
                if s != *serial {
                    continue;
                }
            }
        }
        match matches.subcommand() {
            Some(("get", _)) => {
                let brightness = get_brightness_percent(&mut handle)?;
                println!("brightness {}", brightness);
            }
            Some(("set", sub_matches)) => {
                let brightness = *sub_matches.get_one::<u8>("BRIGHTNESS").expect("required");
                set_brightness_percent(&mut handle, brightness)?;
            }
            Some(("up", sub_matches)) => {
                let step = *sub_matches.get_one::<u8>("step").expect("required");
                let brightness = get_brightness_percent(&mut handle)?;
                let new_brightness = std::cmp::min(100, brightness + step);
                set_brightness_percent(&mut handle, new_brightness)?;
            }
            Some(("down", sub_matches)) => {
                let step = *sub_matches.get_one::<u8>("step").expect("required");
                let brightness = get_brightness_percent(&mut handle)?;
                let new_brightness = brightness.saturating_sub(step);
                set_brightness_percent(&mut handle, new_brightness)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

//
// --- GUI module using Slint ---
//

mod gui {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::collections::HashMap;

    slint::include_modules!();

    struct DisplayHandle {
        handle: Rc<RefCell<hidapi::HidDevice>>,
        serial: String,
        name: String,
    }

    pub fn launch_gui(serial: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        
        let hapi = HidApi::new()?;
        let displays = studio_displays(&hapi)?;
        if displays.is_empty() {
            return Err("No Apple Studio Display found".into());
        }
        
        // Filter displays by serial if specified, otherwise use all displays
        let displays_to_use: Vec<_> = displays
            .into_iter()
            .filter(|d| {
                if let Some(ref serial_filter) = serial {
                    d.serial_number().map(|s| s == serial_filter).unwrap_or(false)
                } else {
                    true
                }
            })
            .collect();
        
        if displays_to_use.is_empty() {
            return Err("No display found with specified serial".into());
        }

        // Open handles for all displays and collect their info
        let mut display_handles = Vec::new();
        for (idx, display) in displays_to_use.iter().enumerate() {
            let handle = hapi.open_path(display.path())?;
            let serial_num = display.serial_number().unwrap_or("Unknown").to_string();
            let display_name = format!("Studio Display {}", idx + 1);
            
            display_handles.push(DisplayHandle {
                handle: Rc::new(RefCell::new(handle)),
                serial: serial_num,
                name: display_name,
            });
        }

        // Read initial brightness from the first display
        let initial_brightness = get_brightness_percent(&mut display_handles[0].handle.borrow_mut())?;

        // Instantiate the Slint UI.
        let ui = BrightnessUI::new().unwrap();
        
        // Set display count and initial values
        ui.set_display_count(display_handles.len() as i32);
        ui.set_master_brightness(initial_brightness as f32);
        
        // Set individual display properties (up to 4 displays)
        for (idx, display) in display_handles.iter().enumerate() {
            let brightness = get_brightness_percent(&mut display.handle.borrow_mut()).unwrap_or(initial_brightness);
            match idx {
                0 => {
                    ui.set_display1_name(display.name.clone().into());
                    ui.set_display1_serial(display.serial.clone().into());
                    ui.set_display1_brightness(brightness as f32);
                }
                1 => {
                    ui.set_display2_name(display.name.clone().into());
                    ui.set_display2_serial(display.serial.clone().into());
                    ui.set_display2_brightness(brightness as f32);
                }
                2 => {
                    ui.set_display3_name(display.name.clone().into());
                    ui.set_display3_serial(display.serial.clone().into());
                    ui.set_display3_brightness(brightness as f32);
                }
                3 => {
                    ui.set_display4_name(display.name.clone().into());
                    ui.set_display4_serial(display.serial.clone().into());
                    ui.set_display4_brightness(brightness as f32);
                }
                _ => break,
            }
        }

        // Create a map for quick serial lookup
        let serial_to_handle: HashMap<String, Rc<RefCell<hidapi::HidDevice>>> = display_handles
            .iter()
            .map(|d| (d.serial.clone(), d.handle.clone()))
            .collect();

        // Clone for callbacks
        let handles_for_master = display_handles.iter().map(|d| d.handle.clone()).collect::<Vec<_>>();
        let handles_for_individual = serial_to_handle.clone();
        
        let ui_weak_master = ui.as_weak();

        // Master brightness callback - always updates all displays
        ui.on_master_brightness_changed(move |new_value: f32| {
            let new_value_u8 = new_value as u8;
            let ui = ui_weak_master.unwrap();
            
            // Update all displays
            for handle in &handles_for_master {
                if let Err(e) = set_brightness_percent(&mut handle.borrow_mut(), new_value_u8) {
                    eprintln!("Failed to set brightness: {:?}", e);
                }
            }
            
            // Update individual display properties in UI
            ui.set_display1_brightness(new_value);
            ui.set_display2_brightness(new_value);
            ui.set_display3_brightness(new_value);
            ui.set_display4_brightness(new_value);
        });

        // Individual brightness callback - updates specific display only
        ui.on_individual_brightness_changed(move |serial: slint::SharedString, new_value: f32| {
            let new_value_u8 = new_value as u8;
            let serial_str = serial.to_string();
            
            if let Some(handle) = handles_for_individual.get(&serial_str) {
                if let Err(e) = set_brightness_percent(&mut handle.borrow_mut(), new_value_u8) {
                    eprintln!("Failed to set brightness for {}: {:?}", serial_str, e);
                }
            }
        });

        // Run the GUI event loop.
        let _ = ui.run();
        Ok(())
    }
}
