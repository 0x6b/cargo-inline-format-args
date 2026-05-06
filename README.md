# cargo-pedantic-lite

A cargo subcommand that runs `cargo clippy --fix` with a curated subset of [`clippy::pedantic`](https://doc.rust-lang.org/clippy/usage.html#clippypedantic) lints — the ones I find genuinely useful, without the noise of the full group.

```console
$ cargo pedantic-lite [clippy options...]
```

It also sets `allow-mixed-uninlined-format-args = false` so `uninlined_format_args` flags cases like `format!("{} {}", a, foo.bar)`.

## Enabled lints

Auto-fixable (applied by `--fix`):

- `clippy::cast_lossless`
- `clippy::explicit_iter_loop`
- `clippy::ignored_unit_patterns`
- `clippy::manual_string_new`
- `clippy::map_unwrap_or`
- `clippy::needless_raw_string_hashes`
- `clippy::redundant_closure_for_method_calls`
- `clippy::semicolon_if_nothing_returned`
- `clippy::single_match_else`
- `clippy::uninlined_format_args`
- `clippy::unnested_or_patterns`

Warning-only (require manual edits):

- `clippy::assigning_clones`
- `clippy::format_push_string`
- `clippy::manual_let_else`
- `clippy::match_same_arms`
- `clippy::needless_pass_by_value`
- `clippy::unused_async`

## Reference

- [Clippy Lints (pedantic group)](https://rust-lang.github.io/rust-clippy/master/index.html?groups=pedantic)

## License

MIT. See [LICENSE](LICENSE) for details.
