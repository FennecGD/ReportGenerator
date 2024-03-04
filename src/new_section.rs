use std::{
    error::Error,
    fs::{read_dir, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use crate::consts::*;

pub fn new_section(
    report_dir: Option<PathBuf>,
    name: Option<String>,
    template: Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Ensure user provided the report path
    let report_path = report_dir.unwrap_or_else(|| {
        eprintln!("ERROR: Report path not provided");
        exit(1);
    });

    // If directory not a valid report, error out
    if File::open(report_path.join("metadata.typ")).is_err() {
        eprintln!("ERROR: Directory not a valid report");
        exit(1);
    }

    // Ensure user provided the name
    let name = name.unwrap_or_else(|| {
        eprintln!("ERROR: name not provided (--name)");
        exit(1);
    });

    let sections_count = read_dir(report_path.join("sections"))?.count();
    let new_section_fname = format!("{}.{name}.typ", sections_count + 1);

    let existing_templates = ["summary"];

    if let Some(ref template) = template {
        if !existing_templates.contains(&template.as_str()) {
            eprintln!("Section not created\nExisting templates: {existing_templates:?}");
            exit(1);
        }
    }

    let mut f = File::options()
        .create_new(true)
        .write(true)
        .open(report_path.join("sections").join(&new_section_fname))?;

    // FIXME: make so it is not necessary to add code here on every template added
    if let Some(template) = template {
        // Handle templates
        match template.as_str() {
            "default" => {
                f.write_all(EXAMPLE_SECTION.as_bytes())?;
            }
            "summary" => {
                f.write_all(EXAMPLE_SUMMARY.as_bytes())?;
            }
            "scope" => {
                f.write_all(EXAMPLE_SCOPE.as_bytes())?;
            }
            "methodology" => {
                f.write_all(EXAMPLE_METHODOLOGY.as_bytes())?;
            }
            _ => ()
        }
    } else {
        // Handle new default section
        f.write_all(EXAMPLE_SECTION.as_bytes())?;
    }

    println!("Added new section \"{new_section_fname}\"");

    Ok(())
}
