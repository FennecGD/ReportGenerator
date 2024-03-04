use std::{
    error::Error,
    fs::{create_dir, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use crate::consts::*;

pub fn new_report(report_dir: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    // Ensure user provided the report path
    let report_path = report_dir.unwrap_or_else(|| {
        eprintln!("ERROR: Report path not provided");
        exit(1);
    });

    // If directory not empty, error out
    if report_path.exists() {
        eprintln!("ERROR: Directory already exists");
        exit(1);
    }

    // Create the file structure
    create_dir(&report_path)?;

    File::create_new(report_path.join("metadata.typ"))?.write_all(T_METADATA.as_bytes())?;

    create_dir(report_path.join("sections"))?;

    File::create_new(report_path.join("sections").join("1.summary.typ"))?
        .write_all(T_SUMMARY.as_bytes())?;
    File::create_new(report_path.join("sections").join("2.scope.typ"))?
        .write_all(T_SCOPE.as_bytes())?;
    File::create_new(report_path.join("sections").join("3.methodology.typ"))?
        .write_all(T_METHODOLOGY.as_bytes())?;
    File::create_new(report_path.join("sections").join("4.example_section.typ"))?
        .write_all(T_SECTION.as_bytes())?;

    create_dir(report_path.join("findings"))?;

    File::create_new(report_path.join("findings").join("1.example_finding.typ"))?
        .write_all(T_FINDING.as_bytes())?;

    println!("New report created successfully");

    Ok(())
}
