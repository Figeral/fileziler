use clap::{Command, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    about = "Fileziler helps help you compress or decompress any file no matter it type or size",
    version
)]
struct Args {
    #[arg(short, long, required = true)]
    opt: Opt,

    #[arg(short, long, required = true)]
    path: String,
}
#[derive(Debug, Clone, ValueEnum, Parser)]
#[command()]
enum Opt {
    compress,
    decompress,
}
fn main() {
    let args = Args::parse();
    start(&args);
}
fn start(args: &Args) {
    println!("{:?}", args.opt);
}
