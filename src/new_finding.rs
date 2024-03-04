use std::{
    error::Error,
    fs::{read_dir, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use crate::consts::EXAMPLE_FINDING;

pub fn new_finding(
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

    let findings_count = read_dir(report_path.join("findings"))?.count();
    let new_finding_fname = format!("{}.{name}.typ", findings_count + 1);

    let existing_templates = ["xss"];

    if let Some(ref template) = template {
        if !existing_templates.contains(&template.as_str()) {
            eprintln!("Finding not created\nExisting templates: {existing_templates:?}");
            exit(1);
        }
    }

    let mut f = File::options()
        .create_new(true)
        .write(true)
        .open(report_path.join("findings").join(&new_finding_fname))?;

    // FIXME: make so it is not necessary to add code here on every template added
    if let Some(template) = template {
        // Handle templates
        match template.as_str() {
            "xss" => {
                f.write_all(include_str!("../templates/findings/xss.typ").as_bytes())?;
            }
            "sql-injection" => {
                f.write_all(include_str!("../templates/findings/sql-injection.typ").as_bytes())?;
            }
            _ => ()
        }
    } else {
        // Handle new default finding
        f.write_all(EXAMPLE_FINDING.as_bytes())?;
    }

    println!("Added new finding \"{new_finding_fname}\"");

    Ok(())
}
