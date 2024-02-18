use std::{
    error::Error,
    fs::{read_to_string, remove_file, OpenOptions},
    io::Write,
    process::Command,
};

mod args;

const REPORT_FILE: &str = "report.pdf";
const TMP_FILE: &str = "tmp.typ";

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::get_args();
    // println!("{args:?}");

    if let Some(command) = args.subcommand {
        match command.as_ref() {
            "new" => (),
            "compile" => {
                let mut report = include_str!("../others/template.typ").to_string();
                let context: Vec<(&str, &str)> = vec![
                    ("report_title", "Pentest Report"),
                    ("date", "January 01, 2024"),
                    ("prepared_for", "Example Data"),
                    ("prepared_by", "Example Data"),
                ];
                for element in context {
                    report = report.replace(&format!("{{{{ {} }}}}", &element.0), &element.1);
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
    }

    Ok(())
}
