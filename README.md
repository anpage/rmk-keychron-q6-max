# RMK-based Firmware for the Keychron Q6 Max

[RMK](https://rmk.rs/) is a feature-rich and easy-to-use keyboard firmware. This project is a more specialized firmware which builds on RMK to act as an alternative for the Keychron Q6 Max keyboard.

This is still experimental and early in development. It is not yet meant to be used on a daily basis. **Don't install it unless you want to tinker.**

Right now only the ANSI/US layout with knob version of the board is supported because that's the one I have. Other models may come, depending on contributions from others.

## Features/Roadmap

See [RMK's roadmap](https://rmk.rs/docs/development/roadmap.html) for upstream features

These are goals for this project, but not necessarily the only goals:

- [x] USB connectivity
- [x] Persistent layout configuration through Vial
- [x] Rotary encoder (knob) support
- [ ] LED Control
- [x] Mac/Windows Layout Switch
- [ ] Bluetooth connectivity

## Requirements

- The [Rust toolchain](https://rustup.rs/) for `thumbv7em-none-eabihf`:
```shell
rustup target add thumbv7em-none-eabihf
```

- flip-link:
```shell
cargo install flip-link
```

## Installing the firmware

The easiest and quickest way to install the firmware from source on an assembled keyboard is through DFU mode.

1. Enter DFU mode on the keyboard:
```
1. Switch the keyboard to USB mode
2. Remove the space bar
3. Press and hold the small button to the left of the switch
4. Plug the keyboard into your computer
```

2. Install cargo-dfu:
```shell
cargo install cargo-dfu
```

3. Build and flash the firmware:
```shell
cargo dfu --release
```

### Debug Probe

If you have a debug probe connected to your board, you can flash using probe-rs.

1. Install [probe-rs](https://probe.rs/docs/getting-started/installation/)

2. Build and flash using debug probe

```shell
cargo run --release
```

## License

The code in this project is licensed under the GNU GPL v3. See COPYING for details.

```
Copyright (C) 2025 Alex Page

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
```
