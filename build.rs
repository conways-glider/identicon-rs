use handlebars::Handlebars;
use serde_json::json;
use std::{fs, process::Command};

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=scripts/README_TEMPLATE.md");
    println!("cargo:rerun-if-changed=build.rs");

    // Get the version
    let git_version = Command::new("git")
        .args(&["describe", "--tags", "--abbrev=0"])
        .output()
        .expect("failed to get version");

    let version_string = String::from_utf8_lossy(&git_version.stdout);
    let parsed_version_string = version_string.trim().replace("v", "");

    println!("Version: {}", parsed_version_string);

    // Get the main example
    let main_example = include_str!("examples/main.rs").trim();

    println!("Main example:\n{}", main_example);

    // Get the README template
    let readme_template = include_str!("templates/README.hbs");

    // Set up handlebars
    let mut reg = Handlebars::new();
    reg.register_escape_fn(handlebars::no_escape);

    // Render the template
    let data = json!({"version": parsed_version_string, "main_example": main_example});
    let template_output = reg
        .render_template(readme_template, &data)
        .expect("Failed to render template");

    println!("Template output:\n{}", template_output);

    // Write the output to the README.md
    fs::write("README.md", template_output).expect("Unable to write file");
}
