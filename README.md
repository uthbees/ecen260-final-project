### Requirements

- A NUCLEO-L476RG board.
- A flashable board with an ESP-32-S2 chip. (Other ESP-32 variants might work as well, or they might require adjustments.)
- A web server.

### Setting up

Everything is standard git/Rust, except that the git hooks are stored in the `.githooks` directory. Run this command
when setting up a new environment:

```bash
git config --local core.hooksPath .githooks
```

### Deploying

- Flash the `cortex-m4-binary` program onto the NUCLEO-L476RG board.
- Flash the `esp32-binary` program onto the ESP-32 board.
- Connect the two boards and their components according to the schematic.
- Put the `web_server` and `web_client` programs onto your web server (and run the `web_server` program).

See the README files of each individual program for more detailed information.
