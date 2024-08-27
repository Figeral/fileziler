use clap::Parser;
mod compression;
mod decompression;
use fileziler::middlewares;
use fileziler::Args;
use fileziler::Opt;
fn main() {
    let args = Args::parse();
    start(&args);
}

fn start(args: &Args) {
    let input_file = middlewares::fetch_file(args);
    let output_file = middlewares::fetch_file(args);
    match args.opt {
        Opt::Compress => {
            compression::compress::encode(&input_file.unwrap(), &output_file.unwrap());
        }
        Opt::Decompress => {
            decompression::decompress::decode(&input_file.unwrap(), &output_file.unwrap());
        }
    }
}
