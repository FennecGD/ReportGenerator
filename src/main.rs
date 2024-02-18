use std::error::Error;

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::get_args();
    // println!("{args:?}");

    if let Some(command) = args.subcommand {
        match command.as_ref() {
            "new" => (),
            "compile" => (),
            _ => (),
        }
    }

    Ok(())
}
