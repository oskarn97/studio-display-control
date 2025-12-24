fn main() {
    slint_build::compile("ui/BrightnessUI.slint").unwrap();
    
    // Generate icon from SVG for Windows
    #[cfg(windows)]
    {
        use std::path::Path;
        
        let svg_path = "icon.svg";
        let ico_path = "icon.ico";
        
        // Only regenerate if SVG exists and ICO doesn't or is older
        if Path::new(svg_path).exists() {
            if !Path::new(ico_path).exists() || 
               std::fs::metadata(svg_path).unwrap().modified().unwrap() > 
               std::fs::metadata(ico_path).unwrap().modified().unwrap() {
                
                println!("cargo:warning=Generating icon.ico from icon.svg...");
                
                // Convert SVG to ICO
                if let Err(e) = generate_ico_from_svg(svg_path, ico_path) {
                    println!("cargo:warning=Failed to generate icon: {}. Using default icon.", e);
                } else {
                    println!("cargo:warning=Icon generated successfully!");
                }
            }
            
            // Embed the icon in the executable
            let mut res = winres::WindowsResource::new();
            res.set_icon(ico_path);
            res.set("FileDescription", "Studio Display Brightness Control");
            res.set("ProductName", "Studio Display Control");
            res.set("CompanyName", "");
            if let Err(e) = res.compile() {
                println!("cargo:warning=Failed to embed icon: {}", e);
            }
        }
    }
}

#[cfg(windows)]
fn generate_ico_from_svg(svg_path: &str, ico_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use resvg::usvg;
    use image::imageops::FilterType;
    
    // Read SVG file
    let svg_data = std::fs::read(svg_path)?;
    
    // Parse SVG
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(&svg_data, &opt)?;
    
    // Icon sizes to generate
    let sizes = [256, 128, 64, 48, 32, 16];
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    
    for &size in &sizes {
        // Render SVG to pixmap
        let pixmap_size = tree.size().to_int_size();
        let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .ok_or("Failed to create pixmap")?;
        
        resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
        
        // Convert to image::RgbaImage
        let img = image::RgbaImage::from_raw(
            pixmap.width(),
            pixmap.height(),
            pixmap.data().to_vec(),
        ).ok_or("Failed to create image")?;
        
        // Resize to target size
        let resized = image::imageops::resize(&img, size, size, FilterType::Lanczos3);
        
        // Create icon image
        let icon_image = ico::IconImage::from_rgba_data(size, size, resized.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
    }
    
    // Write ICO file
    let file = std::fs::File::create(ico_path)?;
    icon_dir.write(file)?;
    
    Ok(())
}

#[cfg(not(windows))]
fn generate_ico_from_svg(_svg_path: &str, _ico_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}