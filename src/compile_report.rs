use std::{
    error::Error,
    fs::{read_dir, read_to_string, remove_file, File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::{exit, Command},
};

use crate::consts::*;
use crate::template::Template;
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

    // User provided output file or DEFAULT_REPORT_FILE as fallback
    let output_file = output.as_deref().unwrap_or(DEFAULT_REPORT_FILE);

    // Use typst to compile the file
    Command::new("typst")
        .args(["compile", TMP_FILE, output_file])
        .spawn()
        .expect("Failed to execute typst\nEnsure you have 'typst' installed on your system")
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

    // Handle sections
    let mut sections = vec![String::new(); read_dir(report_path.join("sections"))?.count()];
    for section in read_dir(report_path.join("sections"))? {
        let section = section?;
        let content = read_to_string(section.path())?;
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
        let content = read_to_string(finding.path())?;
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

    let mut context: Vec<(&str, &str)> = vec![
        ("sections", &sections),
        ("findings", &findings),
        ("current_date", &current_date),
    ];

    // Handle metadata file
    let metadata_file = read_to_string(report_path.join("metadata.typ"))?;
    for line in metadata_file.lines() {
        let split: Vec<&str> = line.split(':').collect();
        if split.len() < 2 {
            continue;
        }
        context.push((split[0], split[1]));
    }

    let report = Template::from_str(MAIN_TEMPLATE).render(&context);

    compile_to_file(&report, &output)?;

    println!("Report compiled successfully");

    Ok(())
}
