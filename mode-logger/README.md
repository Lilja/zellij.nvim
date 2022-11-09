## About

This template is designed for writing [Zellij][zellij] plugins in Rust.

You can learn more about developing plugins in the [Zellij Documentation][docs].

[zellij]: https://github.com/zellij-org/zellij
[docs]: https://zellij.dev/documentation/plugins.html

## Usage

### Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/zellij-org/rust-plugin-template.git --name my-project
cd my-project
```

### Build with `cargo` and Test in Zellij

```sh
# If you don't have Zellij installed already
cargo install zellij
# Building the plugin
cargo build
# Running in Zellij
zellij -l plugin.yaml
```
