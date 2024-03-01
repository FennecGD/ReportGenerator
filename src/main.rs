use chrono::Local;
use std::{
    error::Error,
    fs::{create_dir, read_dir, remove_file, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    process::{exit, Command},
};

mod args;

/*
   report
   - metadata.txt (title, prepared_for, prepared_by, section_order (as TODO))
   - sections (by default: summary.txt, methodology.txt, scope.txt)
   - - section.txt (file name is section name by default (can overwrite with: title:newtitle in the first line), inside is the section content)
   - findings
   - - finding.txt (file name: finding name (ability to ovewrite the name), inside is the finding content + first lines ability to change things)
*/

const REPORT_FILE: &str = "report.pdf";
const TMP_FILE: &str = "tmp.typ";
const REPORT_TEMPLATE: &str = include_str!("../others/template.typ");

const EXAMPLE_METADATA: &str = "title:Example Pentest Report
prepared_for:Example prepared for
prepared_by:Example prepared by";

const EXAMPLE_SECTION: &str = "= Example section
Look at this gorgeus sections content";

const EXAMPLE_FINDING: &str = "= Example finding
Look at this amazing finding";

fn compile_to_file(report: &str) -> Result<(), Box<dyn Error>> {
    // Write report to temporary file
    let mut tmp_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(TMP_FILE)
        .expect("Failed to open temporary file");
    tmp_file.write_all(report.as_bytes())?;

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

    Ok(())
}

fn get_current_date() -> String {
    let date = Local::now();
    date.format("%B %d, %Y").to_string()
}

fn compile_report(report_dir: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    // Ensure user provided the report path
    let report_path = report_dir.unwrap_or_else(|| {
        eprintln!("ERROR: Report path not provided");
        exit(1);
    });

    // If directory doesn't exist, error out
    if !report_path.exists() {
        eprintln!("ERROR: Directory doesn't exist");
        exit(1);
    }

    let mut report_title = "[REPORT TITLE - CHANGE ME]";
    let mut prepared_for = "[PREPARED FOR - CHANGE ME]";
    let mut prepared_by = "[PREPARED BY - CHANGE ME]";
    let mut sections = String::new();
    let mut findings = String::new();

    let mut metadata = String::new();
    File::open(report_path.join("metadata.txt"))?.read_to_string(&mut metadata)?;

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
    for section in read_dir(report_path.join("sections"))? {
        let section = section?;
        let mut content = String::new();
        File::open(section.path())?.read_to_string(&mut content)?;
        sections.push_str(&format!("\n#pagebreak()\n{content}"));
    }

    // Handle findings
    for finding in read_dir(report_path.join("findings"))? {
        let finding = finding?;
        let mut content = String::new();
        File::open(finding.path())?.read_to_string(&mut content)?;
        findings.push_str(&format!("\n#pagebreak()\n{content}"));
    }

    let current_date = get_current_date();

    // TODO: Table of contents, summary, methodology, scope, findings (+evidence)
    // TODO: Special last page (maybe empty? with some text at the bottom?)

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

    compile_to_file(&report)?;

    Ok(())
}

fn new_report(report_dir: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
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

    let mut f_metadata = File::create_new(report_path.join("metadata.txt"))?;
    f_metadata.write_all(EXAMPLE_METADATA.as_bytes())?;

    create_dir(report_path.join("sections"))?;
    let mut f_section = File::create_new(report_path.join("sections").join("example_section.txt"))?;
    f_section.write_all(EXAMPLE_SECTION.as_bytes())?;

    create_dir(report_path.join("findings"))?;
    let mut f_finding = File::create_new(report_path.join("findings").join("example_finding.txt"))?;
    f_finding.write_all(EXAMPLE_FINDING.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::get_args();
    // println!("{args:?}");

    if let Some(command) = args.subcommand {
        match command.as_ref() {
            "new" => {
                new_report(args.dir)?;
            }
            "compile" => {
                compile_report(args.dir)?;
            }
            _ => {
                eprintln!("Incorrect subcommand. Check --help");
                exit(1);
            }
        }
    } else {
        // GUI
        todo!("GUI");
    }

    Ok(())
}
