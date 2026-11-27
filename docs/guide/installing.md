# Installing

With a Rust toolchain installed, install the released crate:

```sh
cargo install koshr
```

Or download a release archive from the
[releases page](https://github.com/startr-trade/koshr/releases). Each release carries one archive
per platform, named `koshr-<version>-<target>`, and a `.sha256` file beside each:

| Platform | Target | Archive |
|---|---|---|
| Linux x86_64 | `x86_64-unknown-linux-musl` | `.tar.gz` |
| Linux arm64 | `aarch64-unknown-linux-musl` | `.tar.gz` |
| macOS Intel | `x86_64-apple-darwin` | `.tar.gz` |
| macOS Apple silicon | `aarch64-apple-darwin` | `.tar.gz` |
| Windows x86_64 | `x86_64-pc-windows-msvc` | `.zip` |

Check the download before you unpack it, then put `koshr` on your `PATH`:

```sh
sha256sum -c koshr-<version>-<target>.tar.gz.sha256
tar -xzf koshr-<version>-<target>.tar.gz
```

The Linux binaries are static, so they run on any distribution.

Check the install:

```sh
koshr --version
```
