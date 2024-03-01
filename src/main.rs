use std::{error::Error, process::exit};

mod args;
mod compile_report;
mod consts;
mod new_report;
mod utils;

// TODO: templates for default finding (+evidence), common vulns, default section

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
                // TODO: new section command (name + optional template)
                todo!("New Section");
            }
            "new-finding" => {
                // TODO: new finding command (name + optional template)
                todo!("New Finding");
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
