#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("heart_inlineBG.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // Do nothing on non-Windows platforms
}
