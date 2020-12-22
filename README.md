# Clippy

[![Clippy Test](https://github.com/rust-lang/rust-clippy/workflows/Clippy%20Test/badge.svg?branch=auto&event=push)](https://github.com/rust-lang/rust-clippy/actions?query=workflow%3A%22Clippy+Test%22+event%3Apush+branch%3Aauto)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/clippy.svg)](#license)

A collection of lints to catch common mistakes and improve your [Rust](https://github.com/rust-lang/rust) code.

[There are over 400 lints included in this crate!](https://rust-lang.github.io/rust-clippy/master/index.html)

Lints are divided into categories, each with a default [lint level](https://doc.rust-lang.org/rustc/lints/levels.html).
You can choose how much Clippy is supposed to ~~annoy~~ help you by changing the lint level by category.

| Category              | Description                                                             | Default level |
| --------------------- | ----------------------------------------------------------------------- | ------------- |
| `clippy::all`         | all lints that are on by default (correctness, style, complexity, perf) | **warn/deny** |
| `clippy::correctness` | code that is outright wrong or very useless                             | **deny**      |
| `clippy::style`       | code that should be written in a more idiomatic way                     | **warn**      |
| `clippy::complexity`  | code that does something simple but in a complex way                    | **warn**      |
| `clippy::perf`        | code that can be written to run faster                                  | **warn**      |
| `clippy::pedantic`    | lints which are rather strict or might have false positives             | allow         |
| `clippy::nursery`     | new lints that are still under development                              | allow         |
| `clippy::cargo`       | lints for the cargo manifest                                            | allow         |

More to come, please [file an issue](https://github.com/rust-lang/rust-clippy/issues) if you have ideas!

The [lint list](https://rust-lang.github.io/rust-clippy/master/index.html) also contains "restriction lints", which are
for things which are usually not considered "bad", but may be useful to turn on in specific cases. These should be used
very selectively, if at all.

Table of contents:

*   [Usage instructions](#usage)
*   [Configuration](#configuration)
*   [Contributing](#contributing)
*   [License](#license)

## Usage

Below are instructions on how to use Clippy as a subcommand, compiled from source
or in Travis CI.

### As a cargo subcommand (`cargo clippy`)

One way to use Clippy is by installing Clippy through rustup as a cargo
subcommand.

#### Step 1: Install rustup

You can install [rustup](https://rustup.rs/) on supported platforms. This will help
us install Clippy and its dependencies.

If you already have rustup installed, update to ensure you have the latest
rustup and compiler:

```terminal
rustup update
```

#### Step 2: Install Clippy

Once you have rustup and the latest stable release (at least Rust 1.29) installed, run the following command:

```terminal
rustup component add clippy
```
If it says that it can't find the `clippy` component, please run `rustup self update`.

#### Step 3: Run Clippy

Now you can run Clippy by invoking the following command:

```terminal
cargo clippy
```

#### Automatically applying Clippy suggestions

Clippy can automatically apply some lint suggestions.
Note that this is still experimental and only supported on the nightly channel:

```terminal
cargo clippy --fix -Z unstable-options
```

#### Workspaces

All the usual workspace options should work with Clippy. For example the following command
will run Clippy on the `example` crate:

```terminal
cargo clippy -p example
```

As with `cargo check`, this includes dependencies that are members of the workspace, like path dependencies.
If you want to run Clippy **only** on the given crate, use the `--no-deps` option like this:

```terminal
cargo clippy -p example -- --no-deps 
```

### Travis CI

You can add Clippy to Travis CI in the same way you use it locally:

```yml
language: rust
rust:
  - stable
  - beta
before_script:
  - rustup component add clippy
script:
  - cargo clippy
  # if you want the build job to fail when encountering warnings, use
  - cargo clippy -- -D warnings
  # in order to also check tests and non-default crate features, use
  - cargo clippy --all-targets --all-features -- -D warnings
  - cargo test
  # etc.
```

Note that adding `-D warnings` will cause your build to fail if **any** warnings are found in your code.
That includes warnings found by rustc (e.g. `dead_code`, etc.). If you want to avoid this and only cause
an error for Clippy warnings, use `#![deny(clippy::all)]` in your code or `-D clippy::all` on the command
line. (You can swap `clippy::all` with the specific lint category you are targeting.)

## Configuration

Some lints can be configured in a TOML file named `clippy.toml` or `.clippy.toml`. It contains a basic `variable =
value` mapping eg.

```toml
blacklisted-names = ["toto", "tata", "titi"]
cognitive-complexity-threshold = 30
```

See the [list of lints](https://rust-lang.github.io/rust-clippy/master/index.html) for more information about which
lints can be configured and the meaning of the variables.

To deactivate the “for further information visit *lint-link*” message you can
define the `CLIPPY_DISABLE_DOCS_LINKS` environment variable.

### Allowing/denying lints

You can add options to your code to `allow`/`warn`/`deny` Clippy lints:

*   the whole set of `Warn` lints using the `clippy` lint group (`#![deny(clippy::all)]`)

*   all lints using both the `clippy` and `clippy::pedantic` lint groups (`#![deny(clippy::all)]`,
    `#![deny(clippy::pedantic)]`). Note that `clippy::pedantic` contains some very aggressive
    lints prone to false positives.

*   only some lints (`#![deny(clippy::single_match, clippy::box_vec)]`, etc.)

*   `allow`/`warn`/`deny` can be limited to a single function or module using `#[allow(...)]`, etc.

Note: `allow` means to suppress the lint for your code. With `warn` the lint
will only emit a warning, while with `deny` the lint will emit an error, when
triggering for your code. An error causes clippy to exit with an error code, so
is useful in scripts like CI/CD.

If you do not want to include your lint levels in your code, you can globally
enable/disable lints by passing extra flags to Clippy during the run:

To allow `lint_name`, run

```terminal
cargo clippy -- -A clippy::lint_name
```

And to warn on `lint_name`, run

```terminal
cargo clippy -- -W clippy::lint_name
```

This also works with lint groups. For example you
can run Clippy with warnings for all lints enabled:
```terminal
cargo clippy -- -W clippy::pedantic
```

If you care only about a single lint, you can allow all others and then explicitly warn on
the lint(s) you are interested in:
```terminal
cargo clippy -- -A clippy::all -W clippy::useless_format -W clippy::...
```

### Specifying the minimum supported Rust version

Projects that intend to support old versions of Rust can disable lints pertaining to newer features by
specifying the minimum supported Rust version (MSRV) in the clippy configuration file.

```toml
msrv = "1.30.0"
```

The MSRV can also be specified as an inner attribute, like below.

```rust
#![feature(custom_inner_attributes)]
#![clippy::msrv = "1.30.0"]

fn main() {
  ...
}
```

You can also omit the patch version when specifying the MSRV, so `msrv = 1.30`
is equivalent to `msrv = 1.30.0`.

Note: `custom_inner_attributes` is an unstable feature so it has to be enabled explicitly.

Lints that recognize this configuration option can be found [here](https://rust-lang.github.io/rust-clippy/master/index.html#msrv)

## Contributing

If you want to contribute to Clippy, you can find more information in [CONTRIBUTING.md](https://github.com/rust-lang/rust-clippy/blob/master/CONTRIBUTING.md).

## License

Copyright 2014-2020 The Rust Project Developers

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)> or the MIT license
<LICENSE-MIT or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)>, at your
option. Files in the project may not be
copied, modified, or distributed except according to those terms.
