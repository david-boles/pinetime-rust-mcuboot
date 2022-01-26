# pinetime-rust-mcuboot
WIP demo project for pure rust playing nicely with MCUBoot/Infinitime OTA

# Usage
# Build the firmware
```
cargo xtask build
```

# Build and flash the firmware
Requires [cargo-flash](https://probe.rs/docs/tools/cargo-flash/).
```
cargo xtask flash
```

# OTA update
The DFU update zip can be found next to the firmware binary after building. It can be flashed using Infinitime, other MCUboot-based firmware, or the Infinitime-based recovery firmware built into the newer bootloader version(s?):
- When booting, hold down the button.
- Wait for acorn to turn red.
- Release button.
- Wait for another boot cycle and for the Infinitime logo to appear.
- Connect and flash as normal.