use std::{
    env::args,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    Command::new("cargo")
        .arg("clippy")
        .args(args().skip(2))
        .args(["--", "-A", "clippy::all", "-W", "clippy::uninlined_format_args"])
        .status()
        .map_or(ExitCode::FAILURE, |s| ExitCode::from(s.code().unwrap_or(1) as u8))
}
