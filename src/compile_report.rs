use std::{
    error::Error,
    fs::{read_dir, remove_file, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    process::{exit, Command},
};

use crate::consts::*;
use crate::utils::get_current_date;

fn compile_to_file(report: &str, output: &Option<String>) -> Result<(), Box<dyn Error>> {
    // Write report to temporary file
    let mut tmp_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(TMP_FILE)
        .expect("Failed to open temporary file");
    tmp_file.write_all(report.as_bytes())?;

    // Close file
    drop(tmp_file);

    let report_output_file = if let Some(file_name) = output {
        file_name
    } else {
        DEFAULT_REPORT_FILE
    };

    // Use typst to compile the file
    Command::new("typst")
        .args(["compile", TMP_FILE, report_output_file])
        .spawn()
        .expect("Failed to execute typst")
        .wait()
        .expect("Failed to wait for typst");

    // Remove the temporary file
    remove_file(TMP_FILE).expect("Failed to remove temporary file");

    Ok(())
}

pub fn compile_report(
    report_dir: Option<PathBuf>,
    output: Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Ensure user provided the report path or use current directory as default
    let report_path = report_dir.unwrap_or_else(|| {
        if File::open("metadata.typ").is_err() {
            eprintln!("ERROR: current directory is not a valid report");
            exit(1);
        }
        ".".into()
    });

    // If directory doesn't exist, error out
    if !report_path.exists() {
        eprintln!("ERROR: Directory doesn't exist");
        exit(1);
    }

    let mut report_title = "[REPORT TITLE - CHANGE ME]";
    let mut prepared_for = "[PREPARED FOR - CHANGE ME]";
    let mut prepared_by = "[PREPARED BY - CHANGE ME]";

    let mut metadata = String::new();
    File::open(report_path.join("metadata.typ"))?.read_to_string(&mut metadata)?;

    // Handle metadata file
    for line in metadata.lines() {
        let split: Vec<&str> = line.split(':').collect();
        if split.len() < 2 {
            continue;
        }
        match split[0] {
            "title" => report_title = split[1],
            "prepared_for" => prepared_for = split[1],
            "prepared_by" => prepared_by = split[1],
            _ => (),
        }
    }

    // Handle sections
    let mut sections = vec![String::new(); read_dir(report_path.join("sections"))?.count()];
    for section in read_dir(report_path.join("sections"))? {
        let section = section?;
        let mut content = String::new();
        File::open(section.path())?.read_to_string(&mut content)?;
        let id = section
            .file_name()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .parse::<usize>()?;
        sections[id - 1] = format!("\n#pagebreak()\n{content}");
    }

    // Handle findings
    let mut findings = vec![String::new(); read_dir(report_path.join("findings"))?.count()];
    for finding in read_dir(report_path.join("findings"))? {
        let finding = finding?;
        let mut content = String::new();
        File::open(finding.path())?.read_to_string(&mut content)?;
        let id = finding
            .file_name()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .parse::<usize>()?;
        findings[id - 1] = format!("\n#pagebreak()\n{content}");
    }

    let sections = sections.join("\n");
    let findings = findings.join("\n");
    let current_date = get_current_date();

    let mut report = REPORT_TEMPLATE.to_owned();
    let context: Vec<(&str, &str)> = vec![
        ("report_title", report_title),
        ("date", &current_date),
        ("prepared_for", prepared_for),
        ("prepared_by", prepared_by),
        ("sections", &sections),
        ("findings", &findings),
    ];
    for element in context {
        report = report.replace(&format!("{{{{ {} }}}}", element.0), element.1);
    }

    compile_to_file(&report, &output)?;

    Ok(())
}
