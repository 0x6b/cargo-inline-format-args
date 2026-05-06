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
clippy::assigning_clones
clippy::cast_lossless
clippy::explicit_iter_loop
clippy::format_push_string
clippy::ignored_unit_patterns
clippy::manual_let_else
clippy::manual_string_new
clippy::map_unwrap_or
clippy::match_same_arms
clippy::needless_pass_by_value
clippy::needless_raw_string_hashes
clippy::redundant_closure_for_method_calls
clippy::semicolon_if_nothing_returned
clippy::single_match_else
clippy::uninlined_format_args
clippy::unnested_or_patterns
clippy::unused_async
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

    // When invoked as `cargo pedantic-lite ...`, cargo passes "pedantic-lite"
    // as the first argument. When invoked directly, that arg is absent.
    let user_args = args().skip(1).skip_while(|a| a == "pedantic-lite");

    let result = Command::new("cargo")
        .arg("clippy")
        .arg("--no-deps")
        .arg("--fix")
        .args(user_args)
        .args(LINT_ARGS.iter())
        .env("CLIPPY_CONF_DIR", temp_dir())
        .status();

    let _ = remove_file(&conf);

    result.map_or(ExitCode::FAILURE, |s| ExitCode::from(s.code().unwrap_or(1) as u8))
}
