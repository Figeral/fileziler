use clap::{Parser, ValueEnum};
use fileziler::Args;

fn main() {
    let args = Args::parse();
    start(&args);
}
fn start(args: &Args) {
    println!("{:?}", args.opt);
}
