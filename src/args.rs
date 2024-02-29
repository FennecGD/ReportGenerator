pico_args_helpgen::define_app! {
    app_name: "Report Generator",
    app_description: "Cyber Security Report Generator",
    app_version: "v0.1.0",

    help_args: "-h, --help",
    version_args: "-V, --version",

    struct AppArgs {
        subcommand: Option<String>, "new, compile", "The subcommand to execute",
        dir: Option<std::path::PathBuf>, "[directory]", "Report directory",
    }
}

fn parse_args() -> Result<AppArgs, pico_args_helpgen::Error> {
    let mut pargs = pico_args_helpgen::Arguments::from_env();

    handle_help_version();

    let args = AppArgs {
        subcommand: pargs.subcommand()?,
        dir: pargs.opt_free_from_str()?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Unexpected argument(s): {:?}", remaining);
        std::process::exit(1);
    }

    Ok(args)
}

pub fn get_args() -> AppArgs {
    parse_args().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    })
}
