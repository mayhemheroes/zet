use std::path::PathBuf;
use std::str::FromStr;
use std::result;

use structopt::StructOpt;

pub fn parse_args() -> Args {
    Args::from_args()
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "setop", about = "find the union or intersection of files considered as sets of lines",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp"),
    after_help=
"Each line is output at most once, no matter how many times it occurs in the file(s). Lines are not sorted, but are printed in the order they occur in the input."
)]
pub struct Args {
    #[structopt(
        name="intersect|union|diff|once",
        raw(next_line_help = "true"), long_help=
"Each operation prints lines meeting a different condition:
    Operation  Prints lines appearing in
    ========== =========================
    intersect: EVERY file
    union:     ANY file
    diff:      the FIRST file, and no other
    once:      exactly ONE file"
    )]
    pub op: OpName,
    #[structopt(
        parse(from_os_str),
        help = "Input files", raw(next_line_help = "true"),
    )]
    pub file: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
pub enum OpName {
    Intersect,
    Diff,
}
impl FromStr for OpName {
    type Err = String;
    fn from_str(s: &str) -> result::Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "intersect" => Ok(OpName::Intersect),
            "diff" => Ok(OpName::Diff),
            _ => Err("Expected intersect, diff, ...".to_owned()),
        }
    }
}

