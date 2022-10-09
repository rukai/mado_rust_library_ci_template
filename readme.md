# Mado Rust Library CI Template

A crate + github actions template that fails CI on rustc and clippy warnings without breakage. (The [not rocket science](https://graydon2.dreamwidth.org/1597.html) rule)

## Setup Steps

1. Fail CI on clippy warnings (which includes rustc warnings) in [your github actions workflow](/.github/workflows/testing.yml#L56)
2. Specify a specific rust release in your [rust-toolchain.toml](/rust-toolchain.toml#L2)
    + clippy and rustc warnings are not stable so we need to pin to a specific rust version to avoid breakage.
3. Commit your [Cargo.lock](/Cargo.lock)
    + Rust libraries can rely on features that are not in your pinned rust release, so we also need to pin dependency versions.
4. Configure a [github actions nightly build](/.github/workflows/nightly_deps_check.yml) that does something like `cargo update; cargo test`, if a failure occurs it raises an issue.
    + Committing the Cargo.lock discourages developers from using the latest patch releases of dependencies so without this automated testing issues would go unnoticed.
    + I consider this step optional as you still get a lot of benefits without it and it does add a lot of complexity to the CI setup. You should always skip this step for projects that do not publish any libraries as it provides no benefit there.
5. To achieve the "not rocket science" rule, this CI template must be paired with something that will enforce every PR is run against the latest "main" commit before merging:
    + small to medium projects can get away with just enabling "Require branches to be up to date before merging" in githubs branch protection rules.
    + but large projects with many PRs merged each day are encouraged to use [bors-ng](https://github.com/bors-ng/bors-ng) as that will scale better.
6. Whenever a new rust version is released manually update the rust version in the [rust-toolchain.toml](/rust-toolchain.toml#L2)
    + New rust releases include speed and diagnostic improvements not to mention new features!

## Advantages

+ Keep your codebase clear of warnings!
+ Keeps development environments uniform between developers (both rust and dependency versions)
  + Experienced rust developers know to always run the latest version of rust and that `cargo update` can resolve strange dependency issues, but this isn't always obvious to newbies.
  + Even among experienced rust developers it eliminates entire classes of problems to know that we are all running the same rust and dependency versions when investigating a particularly gnarly "works on my machine" problem.
  + Associating each commit with rust and dependency versions gives more consistent results when benchmarking.
    + Especially powerful if you have your CI setup to benchmark PRs.
+ Bad dependency updates don't bring development to a halt.
  + Developers can continue their regular work, while one person investigates the dependency issue.
+ Reduces CI cache wastage.
  + Pining dependency versions prevents cache usage from increasing every time a non-breaking dependency update is published.
  + Better cache usage results in better CI build times!

## Disadvantages

+ Manually updating the rust release is a lot of extra effort.
  + Because of this I wouldn't recommend using the template unless your project is merging at least one PR a day. Additionally if development slows down or goes into maintenance mode then going back to a regular CI setup is a good idea.
+ Its easier to ignore an issue about broken dependencies than if you cant merge any PRs.
+ Have to manually delete old toolchains in rustup once they are unused.
+ Increases CI cache wastage.
  + Cache usage drastically increases whenever a new rust version is set in `rust-toolchain.toml`.
  + This template minimizes this problem by using a unique cache whenever `Cargo.lock`, `Cargo.lock` or `rust-toolchain.toml` is changed.

## Used by

I use this template on all of my projects.
e.g:

+ [cargo run-wasm](https://github.com/rukai/cargo-run-wasm)
+ [Winit Input Helper](https://github.com/rukai/winit_input_helper)

## Inspired by

+ <https://fasterthanli.me/articles/my-ideal-rust-workflow> - Describes a simplified version suitable for binaries
+ A discussion with [cwfitzgerald](https://github.com/cwfitzgerald)

## Name

The rust-lang project doesnt use a generic bot like bors-ng for the "not rocket science" rule and instead has their own custom bot called [Homu](https://github.com/rust-lang/homu) ✨

## License

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
+ MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
