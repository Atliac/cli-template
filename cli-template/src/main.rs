use clap::Parser;

/// A simple CLI template that greets you by name.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Your name
    #[arg(short, long, default_value = "world")]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
    println!("cli-template v{}", env!("CARGO_PKG_VERSION"));
}
