# cargo-inline-format-args

A cargo subcommand that runs `cargo clippy --fix -- -A clippy::all -W clippy::uninlined_format_args`.

```console
$ cargo inline-format-args [clippy options...]
```

This lint is in the [`clippy::pedantic`](https://doc.rust-lang.org/clippy/usage.html#clippypedantic) group, but I like it a lot while other pedantic lints sometimes bug me.

## Reference

- [Clippy Lints: uninlined_format_args](https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args)

## Lisense

MIT. See [LICENSE](LICENSE) for details.
