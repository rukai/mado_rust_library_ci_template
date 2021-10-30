# Mado Rust Library CI Template

This is a crate + github actions template that will allow you to fail CI on rustc and clippy warnings while preventing broken builds. (The [not rocket science](https://graydon2.dreamwidth.org/1597.html) rule)

For binaries you can do the same but skip step 4.

## The workflow

1. Fail CI on clippy and rustc warnings in [your github actions workflow](/.github/workflows/testing.yml#L56)
2. Specify a specific rust release in your [rust-toolchain.toml](/rust-toolchain.toml#L2)
    + clippy and rustc warnings are not stable so we need to pin to a specific rust version to avoid breakage
3. Commit your [Cargo.lock](/Cargo.lock)
    + Rust libraries can rely on features that are not in your pinned rust release, so we also need to pin dependency versions.
4. Configure a [github actions nightly build](/.github/workflows/nightly_deps_check.yml) that does something like `cargo update; cargo test`, if a failure occurs it raises an issue.
    + Checking in a Cargo.lock discourages developers from using the latest patch releases of dependencies so without this automated testing issues would go unnoticed.
5. Whenever a new rust version is released manually update the rust version in the [rust-toolchain.toml](/rust-toolchain.toml#L2)
    + New rust releases include speed and diagnostic improvements not to mention new features!

## Benefits

+ Keep your codebase clear of warnings!
+ Keeps development environments uniform between developers (both rust and dependency versions)
  + Experienced rust developers know to always run the latest version of rust and that `cargo update` can resolve strange dependency issues, but this isn't always obvious to newbies.
  + Even among experienced rust developers it eliminates entire classes of problems to know that we are all running the same rust and dependency versions when investigating a particularly gnarly "works on my machine" problem.
  + Associating each commit with rust and dependency versions gives more consistent results when benchmarking.
    + Especially powerful if you have your CI setup to benchmark PRs.
+ Bad dependency updates don't bring development to a halt.
  + Developers can continue their regular work, while one person investigates the dependency issue.

## Downsides

+ Manually updating the rust release is a lot of extra effort.
  + Because of this I wouldn't recommend using the workflow unless your project is merging at least one PR a day.
+ Its easier to ignore an issue about broken dependencies than if you cant merge any PRs.
+ Have to manually delete old toolchains in rustup once they are unused.

## Name

It pairs well with [Homu](https://github.com/barosl/homu) in providing a robust "not rocket science" rule abiding CI ✨ (... although I think everyone actually uses bors-ng these days)

## Inspired by

+ https://fasterthanli.me/articles/my-ideal-rust-workflow - Describes a simplified version suitable for binaries
+ A discussion with [cwfitzgerald](https://github.com/cwfitzgerald)

## Used by

I don't actually have a large scale library to try this on. 😅
So for now I'm just trying it on [Winit Input Helper](https://github.com/rukai/winit_input_helper) even though it is not even close to the scale of development at which I would recommend this approach.
I would like to hear if there are any libraries doing something similar and I'll mention them here.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
