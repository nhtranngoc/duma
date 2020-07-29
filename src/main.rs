use structopt::StructOpt;
mod token;
mod interpreter;

use interpreter::Interpreter;

// struct for Cli argument(s)
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)?;

    let mut interpreter = Interpreter::new(content.into());
    let result = interpreter.expr();

    println!("{}", result);

    Ok(())
}
