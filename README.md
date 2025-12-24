# Studio Display Control

**Professional brightness control for Apple Studio Display(s) on Windows PC**

A modern, feature-rich tool to control the brightness of one or multiple Apple Studio Display(s) connected to your Windows computer. Features a beautiful GUI with multi-display support and intuitive controls.

_Forked from `juliuszint/asdbctl` and `himbeles/studi`, significantly enhanced with modern UI and multi-display capabilities._

![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

### 🎨 Modern Graphical User Interface
- **Beautiful, intuitive design** optimized for Windows 11
- **HiDPI/5K support** - Perfect scaling on high-resolution displays
- **Custom icons** - Professional look without emoji dependencies
- **Compact layout** - All controls visible without scrolling (up to 4 displays)

### 🖥️ Multi-Display Support
- **Master Control** - Adjust all displays simultaneously with one slider
- **Individual Controls** - Fine-tune each display separately
- **Real-time sync** - Changes apply instantly to all selected displays
- **Display information** - Shows serial numbers and connection status
- **Up to 4 displays** - Full support for complex multi-monitor setups

### ⚡ Command Line Interface
- **Quick adjustments** - Change brightness without opening the GUI
- **Scriptable** - Integrate into your workflows
- **Batch operations** - Control multiple displays at once
- **Serial-specific** - Target individual displays by serial number

## 📥 Installation

### Option 1: Download Pre-Built Binary (Recommended)

Download the latest `studi.exe` from the [Releases](https://github.com/oskarn97/studio-display-control/releases) page.

No installation required - just double-click to run!

### Option 2: Build from Source

**Requirements:**
- [Rust](https://rustup.rs/) (latest stable)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with C++ support

**Build:**
```bash
git clone https://github.com/oskarn97/studio-display-control.git
cd studio-display-control
cargo build --release
```

The compiled binary will be in `target/release/studi.exe`

## 🚀 Quick Start

### Graphical Interface

Simply launch `studi.exe` - the GUI opens automatically if no command-line arguments are provided.

**UI Features:**
- 🎚️ **Master Control** - Top slider adjusts all displays at once
- 🖥️ **Individual Controls** - Per-display sliders for fine-tuning
- 📊 **Live Percentage Display** - See exact brightness values
- ℹ️ **Connection Status** - Green indicator shows active displays

### Command Line

```bash
# Get current brightness
studi.exe get

# Set brightness to 75%
studi.exe set 75

# Increase by 10% (default step)
studi.exe up

# Decrease by 5% (custom step)
studi.exe down --step 5

# Control specific display
studi.exe --serial ABC123456 set 50

# Show all options
studi.exe --help
```

## 📖 Usage Examples

### Docking Station Workflow
```bash
# Set all displays to 80% when docking
studi.exe set 80

# Or use the GUI for visual control
studi.exe
```

### Multi-Display Setup
1. Open `studi.exe`
2. Use **Master Control** for quick synchronization
3. Use **Individual Controls** to adjust each display separately
4. Close the app - settings stay until next adjustment

### Automated Scripts
```bash
# Morning routine - bright displays
studi.exe set 90

# Evening routine - dim displays
studi.exe set 40

# Gaming mode - max brightness
studi.exe set 100
```

## 🎯 Command Reference

### Commands

| Command | Description | Example |
|---------|-------------|---------|
| `get` | Get current brightness (%) | `studi.exe get` |
| `set` | Set brightness to specific value | `studi.exe set 75` |
| `up` | Increase brightness | `studi.exe up --step 10` |
| `down` | Decrease brightness | `studi.exe down --step 5` |

### Options

| Option | Description | Example |
|--------|-------------|---------|
| `-s, --serial` | Target specific display by serial | `--serial ABC123` |
| `-v, --verbose` | Enable debug output | `-v` or `-vv` |
| `-h, --help` | Show help information | `--help` |

## 🔧 Technical Details

### Display Communication

The tool communicates with Apple Studio Display using **HID (Human Interface Device) protocol**:

- **Vendor ID:** `0x05ac` (Apple Inc.)
- **Product ID:** `0x1114` (Studio Display)
- **Interface:** `0x7` (Control Interface)
- **Brightness Range:** 400-60000 nits (internal units)

### HID Report Structure

```
Report ID: 0x01
Data: [report_id, brightness_low, brightness_high, 0x00, 0x00, 0x00, 0x00]
```

The brightness value is encoded in **little-endian** format (LSB first).

### USB Configuration

Apple Studio Display can operate in 3 USB configurations:
- **macOS:** Uses interface `0xc`
- **Windows/Linux:** Uses interface `0x7`

This difference is automatically handled by the tool.

## 🏗️ Architecture

### UI Technology
- **Framework:** [Slint](https://slint.rs/) - Modern, GPU-accelerated UI
- **Rendering:** Native Windows controls with hardware acceleration
- **Scaling:** Automatic HiDPI/5K display support

### Core Components
- **HID Backend:** Direct USB HID communication via [hidapi](https://github.com/libusb/hidapi)
- **CLI Parser:** [clap](https://github.com/clap-rs/clap) for command-line interface
- **Icon Generation:** Automatic `.ico` generation from SVG at build time

## 🐛 Troubleshooting

### "No Apple Studio Display found"

**Solutions:**
1. Verify display is connected and powered on
2. Check Device Manager - display should appear under "Human Interface Devices"
3. Try reconnecting the USB cable
4. Restart the application

### Display not responding to brightness changes

**Solutions:**
1. Disconnect and reconnect the display
2. Try restarting your computer
3. Update display firmware (if available through macOS)
4. Run with verbose logging: `studi.exe -vv get`

### Multiple displays but only one responds

**Check:**
- Each display should have a unique serial number
- Use `studi.exe -v get` to see all detected displays
- Try controlling them individually: `studi.exe --serial <SERIAL> set 50`

## 📝 Changelog

### Version 2.0 (Current)
- ✨ Complete UI redesign with modern look
- 🖥️ Full multi-display support (up to 4 displays)
- 🎚️ Master control for synchronized brightness
- 🎨 Individual controls for per-display adjustment
- 🎯 HiDPI/5K display support
- 🖼️ Custom application icon
- 📊 Real-time brightness percentage display
- 🎨 Professional custom icons (no emoji issues)
- ⚡ Improved performance and stability

### Version 1.0
- Basic GUI with single slider
- Multi-display support (all displays linked)
- CLI commands (get/set/up/down)

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs via GitHub Issues
- Submit feature requests
- Create pull requests
- Improve documentation

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

This project builds upon:
- [juliuszint/asdbctl](https://github.com/juliuszint/asdbctl) - Original CLI tool
- [himbeles/studi](https://github.com/himbeles/studi) - Added initial UI
- [LG-ultrafine-brightness](https://github.com/ycsos/LG-ultrafine-brightness) - HID protocol research
- [acdcontrol](https://github.com/yhaenggi/acdcontrol) - Technical reference

## 🔗 Links

- **Repository:** https://github.com/oskarn97/studio-display-control
- **Issues:** https://github.com/oskarn97/studio-display-control/issues
- **Releases:** https://github.com/oskarn97/studio-display-control/releases

---

**Made with ❤️ for Studio Display users on Windows**
