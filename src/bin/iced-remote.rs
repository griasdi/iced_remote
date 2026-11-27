//! `iced-remote ctl ...`, `iced-remote sheet ...`,
//! `iced-remote install ...`, `iced-remote help`.

use std::io::Write;

const USAGE: &str =
    "usage: iced-remote ctl [--wait MS] [--keep-going] [--window N] <socket> <command> [args...] | -
       iced-remote sheet [--cols N] [--labels a,b,c] [--max-width PX] OUT.png IN.png...
       iced-remote install [--agent claude,omp] [--yes] [DIR]
       iced-remote help";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args.split_first() {
        Some((command, rest)) if command == "ctl" => iced_remote::cli::ctl(rest),
        Some((command, rest)) if command == "sheet" => iced_remote::cli::sheet(rest),
        Some((command, rest)) if command == "install" => iced_remote::cli::install(rest),
        Some((command, [])) if command == "help" => {
            let _ = writeln!(std::io::stdout().lock(), "{}", iced_remote::cli::help());
            0
        }
        _ => {
            eprintln!("{USAGE}");
            2
        }
    };
    std::process::exit(code);
}
