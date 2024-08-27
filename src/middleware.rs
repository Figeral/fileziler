pub mod middlewares {

    use path::Path;
    use std::*;

    use super::Args;
    pub fn fetch_file(arg: &Args) -> Result<String, &str> {
        let path: &Path = Path::new(&(arg.path));
        match &path.exists() {
            true => Ok(path.display().to_string()),
            _ => Err("filed not found or does not exit. check Your path"),
        }
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
    Compress,
    Decompress,
}
