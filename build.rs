extern crate winres;


fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("cosmic".into());
    slint_build::compile_with_config("ui/app-window.slint", config).unwrap();
    // slint_build::compile("ui/app-window.slint").expect("Slint build failed");
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ui/icons/njord.ico"); // Replace this with the filename of your .ico file.
        res.compile().unwrap();
    }
}
