use egui;
use std::sync::Arc;


pub fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Option 1: Embed a font file directly into the binary (recommended for distribution)
    // Download a font that supports Asian characters (e.g., Noto Sans CJK) and place it in src/
    // Uncomment the following lines and replace "your_asian_font.ttf" with the actual filename:
    

    let font_data = include_bytes!("./NotoSansSC-Regular.ttf"); 
    fonts.font_data.insert(
        "noto_sans_cjk".to_owned(),
        Arc::new(egui::FontData::from_static(font_data)),
    );
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "noto_sans_cjk".to_owned());
    fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "noto_sans_cjk".to_owned());
    ctx.set_fonts(fonts);
    return;


    // Option 2: Load system fonts 
    // This tries to find and load Asian-compatible fonts from the system

    /* 
    let font_loaded;
    
    #[cfg(target_os = "windows")]
    {
        font_loaded = load_windows_fonts(&mut fonts);
    }
    
    #[cfg(target_os = "macos")]
    {
        font_loaded = load_macos_fonts(&mut fonts);
    }
    
    #[cfg(target_os = "linux")]
    {
        font_loaded = load_linux_fonts(&mut fonts);
    }
    
    // For platforms not covered above
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        font_loaded = false;
    }

    if font_loaded {
        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
        println!("Asian font support enabled");
    } else {
        println!("Warning: No Asian fonts found. Asian characters may not display correctly.");
        println!("To ensure Asian character support, download a CJK font and embed it using Option 1 in the code.");
    }*/
}

/* 
#[cfg(target_os = "windows")]
fn load_windows_fonts(fonts: &mut egui::FontDefinitions) -> bool {
    // Try multiple Windows system fonts that support Asian characters
    let font_paths = [
        "C:/Windows/Fonts/msgothic.ttc",  // MS Gothic (Japanese)
        "C:/Windows/Fonts/simhei.ttf",    // SimHei (Chinese Simplified)
        "C:/Windows/Fonts/simsun.ttc",    // SimSun (Chinese Simplified)
        "C:/Windows/Fonts/malgun.ttf",    // Malgun Gothic (Korean)
        "C:/Windows/Fonts/meiryo.ttc",    // Meiryo (Japanese)
        "C:/Windows/Fonts/msyh.ttc",      // Microsoft YaHei (Chinese)
        "C:/Windows/Fonts/yugothic.ttf",  // Yu Gothic (Japanese)
    ];
    
    for (i, font_path) in font_paths.iter().enumerate() {
        if let Ok(font_data) = std::fs::read(font_path) {
            let font_name = format!("asian_font_{}", i);
            fonts.font_data.insert(
                font_name.clone(),
                Arc::new(egui::FontData::from_owned(font_data)),
            );
            
            // Insert at the beginning of the font family list
            fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, font_name.clone());
            fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, font_name);
            
            return true; // Use the first available font
        }
    }
    false
}

#[cfg(target_os = "macos")]
fn load_macos_fonts(fonts: &mut egui::FontDefinitions) -> bool {
    let font_paths = [
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Apple SD Gothic Neo.ttc",
    ];
    
    for (i, font_path) in font_paths.iter().enumerate() {
        if let Ok(font_data) = std::fs::read(font_path) {
            let font_name = format!("asian_font_{}", i);
            fonts.font_data.insert(
                font_name.clone(),
                Arc::new(egui::FontData::from_owned(font_data)),
            );
            
            fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, font_name.clone());
            fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, font_name);
            
            return true;
        }
    }
    false
}

#[cfg(target_os = "linux")]
fn load_linux_fonts(fonts: &mut egui::FontDefinitions) -> bool {
    let font_paths = [
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    ];
    
    for (i, font_path) in font_paths.iter().enumerate() {
        if let Ok(font_data) = std::fs::read(font_path) {
            let font_name = format!("asian_font_{}", i);
            fonts.font_data.insert(
                font_name.clone(),
                Arc::new(egui::FontData::from_owned(font_data)),
            );
            
            fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, font_name.clone());
            fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, font_name);
            
            return true;
        }
    }
    false
}
*/