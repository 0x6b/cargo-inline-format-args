use std::{
    env::{args, temp_dir},
    fs::{remove_file, write},
    process::{Command, ExitCode},
    sync::LazyLock,
};

const CLIPPY_CONF: &str = "
allow-mixed-uninlined-format-args = false
";

const LINT_RULES: &str = "
clippy::uninlined_format_args
";

static LINT_ARGS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    ["--", "--allow", "clippy::all"]
        .into_iter()
        .chain(
            LINT_RULES
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty() && !s.starts_with('#') && !s.starts_with("//"))
                .flat_map(|lint| ["--warn", lint]),
        )
        .collect()
});

fn main() -> ExitCode {
    let conf = temp_dir().join("clippy.toml");

    if write(&conf, CLIPPY_CONF).is_err() {
        return ExitCode::FAILURE;
    }

    let result = Command::new("cargo")
        .arg("clippy")
        .arg("--no-deps")
        .arg("--fix")
        .args(args().skip(2)) // skip "cargo inline-format-args"
        .args(LINT_ARGS.iter())
        .env("CLIPPY_CONF_DIR", temp_dir())
        .status();

    let _ = remove_file(&conf);

    result.map_or(ExitCode::FAILURE, |s| ExitCode::from(s.code().unwrap_or(1) as u8))
}
