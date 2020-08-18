use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliOptions {
    pub command: String,

    #[structopt(parse(from_os_str))]
    pub output: Option<PathBuf>
}

pub enum Task {
    Init,
    Build,
    None
}