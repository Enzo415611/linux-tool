use slint_build::CompilerConfiguration;

fn main() {
    slint_build::compile_with_config(
        "ui/app-window.slint",
        CompilerConfiguration::new()
            .with_style("material-dark".to_string()))
            .expect("Slint build failed");
    // slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
