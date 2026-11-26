use aeris_config::ConfigDocument;
use std::path::Path;
use std::process::ExitCode;

const HELP: &str = "Aeris Core (development scaffold)\nUsage: aeris <help|version|config check|test FILE>\nNetworking commands are not implemented yet.";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [] => println!("{HELP}"),
        [arg] if arg == "help" || arg == "--help" || arg == "-h" => println!("{HELP}"),
        [arg] if arg == "version" || arg == "--version" || arg == "-V" => {
            println!("Aeris Core {}", env!("CARGO_PKG_VERSION"));
        }
        [config, check, path] if config == "config" && (check == "check" || check == "test") => {
            match ConfigDocument::read(Path::new(path)).and_then(|doc| doc.check()) {
                Ok(report) => {
                    println!("Structural check passed; runtime compatibility NOT verified.");
                    println!(
                        "Unchecked top-level fields: {}",
                        report.unchecked_top_level_fields
                    );
                    println!(
                        "Unchecked rule matchers: {}",
                        report.unchecked_rule_matchers
                    );
                    println!("Rules following MATCH: {}", report.rules_after_match);
                }
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            }
        }
        _ => {
            eprintln!("Unsupported command. Run aeris help.");
            return ExitCode::from(2);
        }
    }
    ExitCode::SUCCESS
}
