use std::io;
use std::process::ExitCode;

use io::Write;

use clap::Parser as _;

use rs_ua2json::ua2parsed2json2writer;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    user_agent: String,
}

fn sub() -> Result<(), io::Error> {
    let args = Cli::parse();
    let parser = woothee::parser::Parser::new();
    let mut ol = io::stdout().lock();
    let ua: &str = &args.user_agent;
    ua2parsed2json2writer(&parser, ua, &mut ol)?;
    ol.flush()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
