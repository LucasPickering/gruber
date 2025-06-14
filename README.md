# gruber

[![Test CI](https://github.com/github/docs/actions/workflows/test.yml/badge.svg)](https://github.com/LucasPickering/gruber/actions)
[![crates.io](https://img.shields.io/crates/v/gruber.svg)](https://crates.io/crates/gruber)

## Hardware

- [Raspberry Pi 3B+](https://www.raspberrypi.com/products/raspberry-pi-3-model-b-plus/)
- [Pimoroni HyperPixel 4.0 Square](https://www.adafruit.com/product/4499)

## Development

I haven't figured out to run this locally, it needs some hardware mocking. Usually it's easiest to just run it on the Pi.

### Prerequisites

- Cross-compilation uses [cross](https://github.com/cross-rs/cross/blob/main/docs/getting-started.md#installing-cross)
- Docker installed and running
- Docker [authenticated to GitHub's image registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#authenticating-to-the-container-registry)

### Pi Setup

From a fresh RPi OS installation, you'll need to set it to boot to the console insetad of desktop.

### Deployment

The executable is cross-compiled for the Raspberry Pi, then copied over with a script. Make sure you installed the correct linker in the prerequisites.

To run the program on the Pi with a live SSH session, run:

```sh
./build.sh
```

To spawn the systemctl service and run it in the background:

```sh
./build.sh --release
```
