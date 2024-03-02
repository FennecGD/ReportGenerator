use std::{error::Error, process::exit};

mod args;
mod consts;
mod utils;
mod template;

mod compile_report;
mod new_report;
mod new_section;
mod new_finding;

// TODO: templates for default finding (+evidence), common vulns, default section
// TODO: better looking template

/*
   report
   - metadata.typ
   - sections
   - - 1.summary.typ
   - - 2.scope.typ
   - - 3.methodology.typ
   - - 4.section.typ
   - findings
   - - 1.finding.typ
*/

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::get_args();

    if let Some(command) = args.subcommand {
        match command.as_ref() {
            "new" => {
                new_report::new_report(args.dir)?;
            }
            "compile" => {
                compile_report::compile_report(args.dir, args.output)?;
            }
            "new-section" => {
                new_section::new_section(args.dir, args.name, args.template)?;
            }
            "new-finding" => {
                new_finding::new_finding(args.dir, args.name, args.template)?;
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
