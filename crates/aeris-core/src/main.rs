use std::process::ExitCode;

const HELP: &str = "Aeris Core (development scaffold)\nUsage: aeris <help|version>\nNetworking commands are not implemented yet.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => println!("{HELP}"),
        [arg] if matches!(arg.as_str(), "help" | "--help" | "-h") => println!("{HELP}"),
        [arg] if matches!(arg.as_str(), "version" | "--version" | "-V") => {
            println!("Aeris Core {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Unsupported command. Run aeris help.");
            return ExitCode::from(2);
        }
    }
    ExitCode::SUCCESS
}
