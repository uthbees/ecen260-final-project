Embedded Rust for the NUCLEO-L476RG board. (Will not work with other boards without adjustments.)

### Setting up

Everything is standard git/Rust, except that the git hooks are stored in the `.githooks` directory. Run this command
when setting up a new environment:

```bash
git config --local core.hooksPath .githooks
```

### Requirements

Install probe-rs: https://probe.rs/docs/getting-started/installation/

Connect the board via USB, then run with cargo:
```bash
cargo run
```
