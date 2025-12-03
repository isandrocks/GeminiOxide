#[cfg(windows)]
extern crate winres;

fn main() {
    // Load .env file at build time
    dotenv::dotenv().ok();
    
    // Read GEMINI_API_KEY and make it available to env! macro
    if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
        let trimmed_key = api_key.trim();
        
        if trimmed_key.is_empty() {
            println!("cargo:warning=GEMINI_API_KEY is empty in .env file!");
            println!("cargo:rustc-env=GEMINI_API_KEY=");
        } else if trimmed_key.contains("your-actual-api-key-here") || trimmed_key.contains("placeholder") {
            println!("cargo:warning=GEMINI_API_KEY appears to be a placeholder value!");
            println!("cargo:rustc-env=GEMINI_API_KEY={}", trimmed_key);
        } else {
            println!("cargo:warning=GEMINI_API_KEY loaded successfully (length: {})", trimmed_key.len());
            println!("cargo:rustc-env=GEMINI_API_KEY={}", trimmed_key);
        }
    } else {
        println!("cargo:warning=GEMINI_API_KEY not found in .env file. Set it before building.");
        println!("cargo:rustc-env=GEMINI_API_KEY=");
    }
    
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("heart_inlineBG.ico");
        res.compile().unwrap();
    }
}
