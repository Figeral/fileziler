pub mod middlewares {

    use path::Path;
    use std::*;

    use super::Args;
    pub fn fetch_file(arg: &Args) -> &Path {
        let path: &Path = Path::new(&(arg.path));
        path
    }
}

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    about = "Fileziler helps help you compress or decompress any file no matter it type or size",
    version
)]
pub struct Args {
    #[arg(short, long, required = true)]
    pub opt: Opt,

    #[arg(short, long, required = true)]
    pub path: String,
}
#[derive(Debug, Clone, ValueEnum, Parser)]
#[command()]
pub enum Opt {
    compress,
    decompress,
}
