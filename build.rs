use slint_build::CompilerConfiguration;


fn main() {
    let config = CompilerConfiguration::new().with_style("cosmic".into());
    slint_build::compile_with_config("ui/app-window.slint", config)
        .expect("Slint build failed");
}
