use handlebars::Handlebars;
use serde_json::json;
use std::fs;

fn main() {
    // Add generic files to rerun list.
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");

    // Generate the README
    generate_readme();
}

fn generate_readme() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=templates/README.hbs");

    // Get the version
    let identicon_rs_version = env!("CARGO_PKG_VERSION");

    println!("Version: {}", identicon_rs_version);

    // Get the main example
    let main_example = include_str!("examples/main.rs").trim();

    println!("Main example:\n{}", main_example);

    // Get the README template
    let readme_template = include_str!("templates/README.hbs");

    // Set up handlebars
    let mut reg = Handlebars::new();
    reg.register_escape_fn(handlebars::no_escape);

    // Render the template
    let data = json!({"version": identicon_rs_version, "main_example": main_example});
    let template_output = reg
        .render_template(readme_template, &data)
        .expect("Failed to render template");

    println!("Template output:\n{}", template_output);

    // Write the output to the README.md
    fs::write("README.md", template_output).expect("Unable to write file");
}
