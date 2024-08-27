use clap::Parser;

use fileziler::middlewares;
use fileziler::Args;
fn main() {
    let args = Args::parse();
    start(&args);
}

fn start(args: &Args) {
    let file = middlewares::fetch_file(args);
    println!("{:?}", file.ok());
}
