use chrono::Local;
use std::{
    error::Error,
    fs::{create_dir, remove_file, File, OpenOptions},
    io::Write,
    process::{exit, Command},
};

mod args;

const REPORT_FILE: &str = "report.pdf";
const TMP_FILE: &str = "tmp.typ";
const REPORT_TEMPLATE: &str = include_str!("../others/template.typ");

fn compile_report(report: &str) {
    // Write report to temporary file
    let mut tmp_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(TMP_FILE)
        .expect("Failed to open temporary file");
    tmp_file.write_all(report.as_bytes()).unwrap();

    // Close file
    drop(tmp_file);

    // Use typst to compile the file
    Command::new("typst")
        .args(["compile", TMP_FILE, REPORT_FILE])
        .spawn()
        .expect("Failed to execute typst")
        .wait()
        .expect("Failed to wait for typst");

    // Remove the temporary file
    remove_file(TMP_FILE).expect("Failed to remove temporary file");
}

fn get_current_date() -> String {
    let date = Local::now();
    date.format("%B %d, %Y").to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::get_args();
    // println!("{args:?}");

    if let Some(command) = args.subcommand {
        match command.as_ref() {
            "new" => {
                // Ensure user provided the report path
                let report_path = args.dir.unwrap_or_else(|| {
                    eprintln!("ERROR: Report path not provided");
                    exit(1);
                });

                // If directory not empty, then error out
                if report_path.exists() {
                    eprintln!("ERROR: Directory already exists");
                    exit(1);
                }

                /*
                   report
                   - metadata.txt (title, prepared_for, prepared_by, section_order (as TODO))
                   - sections (by default: summary.txt, methodology.txt, scope.txt)
                   - - section.txt (file name is section name by default (can overwrite with: title:newtitle in the first line), inside is the section content)
                   - findings
                   - - finding.txt (file name: finding name (ability to ovewrite the name), inside is the finding content + first lines ability to change things)
                */

                // Create the file structure
                create_dir(&report_path)?;

                let mut f_metadata = File::create_new(&report_path.join("metadata.txt"))?;
                f_metadata.write_all(
                    b"title:Example Report
prepared_for:Example prepared for
prepared_by:Example prepared by",
                )?;

                create_dir(&report_path.join("sections"))?;
                let mut f_section =
                    File::create_new(&report_path.join("sections").join("example_section.txt"))?;
                f_section.write_all(
                    b"title:Example section
Look at this gorgeus sections content",
                )?;

                create_dir(&report_path.join("findings"))?;
                let mut f_finding =
                    File::create_new(&report_path.join("findings").join("example_finding.txt"))?;
                f_finding.write_all(
                    b"title:Example finding
Look at this amazing finding",
                )?;
            }
            "compile" => {
                let current_date = get_current_date();
                let report_title = "Pentest Report";
                let prepared_for = "Example Data";
                let prepared_by = "Example Data";

                // TODO: Table of contents, summary, methodology, scope, findings (+evidence)
                // TODO: Special last page (maybe empty? with some text at the bottom?)
                // TODO: Maybe a universal way of adding new sections? so to not implement all of
                // them separately?

                let mut report = REPORT_TEMPLATE.to_owned();
                let context: Vec<(&str, &str)> = vec![
                    ("report_title", report_title),
                    ("date", &current_date),
                    ("prepared_for", prepared_for),
                    ("prepared_by", prepared_by),
                ];
                for element in context {
                    report = report.replace(&format!("{{{{ {} }}}}", element.0), element.1);
                }
                compile_report(&report);
            }
            _ => {
                eprintln!("Incorrect subcommand. Check --help");
                std::process::exit(1);
            }
        }
    } else {
        // GUI
        todo!("GUI");
    }

    Ok(())
}
