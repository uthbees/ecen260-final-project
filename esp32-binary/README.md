This program goes on the ESP-32 board.

### Requirements

Follow the directions here: https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites

Connect the board via USB, then run with cargo:
```bash
cargo run
```

You may need to do some troubleshooting. I had to use version 2.1.0 of `espflash`, as well as resetting my board before connecting to it by holding down the boot button, pressing the reset button, and releasing the boot button. (And then pressing the reset button again afterward to get the program to actually run.)
