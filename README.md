# Goal

Get all 25 solutions for AoC 2022 using an nrf51822xxAC. The rules are simple, all input files should be on the device
and parsing the data beforehand is *not* allowed (the full input must be stored lossless). This
will be a challenge for sure.

<details><summary>Completed days.</summary>

| day | part 1 (us) | part 2 (us) | day | part 1 (us) | part 2 (us) |
|-----|-------------|-------------|-----|-------------|-------------|
| 1   | 11,964      | 11,964      | 14  |             |             |
| 2   | 13,364      | 13,365      | 15  |             |             |
| 3   | 29,327      | 28,986      | 16  |             |             |
| 4   | 20,453      | 19,809      | 17  |             |             |
| 5   | 29,233      | 33,276      | 18  |             |             |
| 6   | 6,107       | 10,418      | 19  |             |             |
| 7   | 17,689      | 17,873      | 20  |             |             |
| 8   | 168,624     | 167,650     | 21  |             |             |
| 9   | 82,059,566  |             | 22  |             |             |
| 10  |             |             | 23  |             |             |
| 11  |             |             | 24  |             |             |
| 12  |             |             | 25  |             |             |
| 13  |             |             |     |             |             |

</details>

# Required Tools

| Tool                                                              | Installation Command                                        |
|-------------------------------------------------------------------|-------------------------------------------------------------|
| [rustup](https://www.rust-lang.org/tools/install)                 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs` |
| llvm-tools-preview                                                | `rustup component add llvm-tools-preview`                   |
| thumbv6m-none-eabi                                                | `rustup target add thumbv6m-none-eabi`                      |
| [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) | `cargo install cargo-binutils`                              |
| [cargo-embed](https://github.com/probe-rs/cargo-embed)            | `cargo install cargo-embed`                                 |
| [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat)          | `cargo install cargo-bloat`                                 |
| OpenOCD                                                           | `sudo apt install openocd`                                  |

# Setting up Environment

* Set USB permissions for your probe by running ``lsusb | grep -i <probe>``.
    1. Check the current permissions by running `ls -l /dev/bus/usb/<Bus>/<Device>`.
    2. If it's not ``crw-rw-rw-`` continue to the next step.
    3. Create a file in ``/etc/udev/rules.d`` such as ``69-<probe>.rules``.
    4. Add the following rule to the
       file ``SUBSYSTEM=="usb",ATTR{idVendor}=="<idVendor>",ATTR{idProduct}=="<idProduct>",MODE:="666"`` with the IDs
       you found before.
    5. Reload udev rules by running ``sudo udevadm control --reload-rules``.
    6. Re-insert your probe and repeat the above steps until the permissions are correct.
* Erase any potentially protected memory through OpenOCD.
    1. Create an ``openocd.cfg`` file and add an interface and a target to it,
       e.g. ``source [find interface/stlink.cfg]`` and our target ``source [find target/nrf51.cfg]``.
    2. Connect your probe to the board and open openocd: ``openocd``.
    3. Open a second terminal and connect through telnet (usually port 4444): ``telnet localhost <PORT>``.
    4. Halt the chip by running ``halt``.
    5. Erase all the sectors on bank 0 by running ``flash erase_sector 0 0 last``.

# Building, Flashing and Debugging

```
# Build
cargo build --release

# Flash, Reset and open RTT
cargo embed --release flash

# Get overview of binary size
cargo size --release

# Get detailed analysis of binary size (only for dev profile).
cargo bloat --release
```

# Random

```
cargo readobj --file-headers

# open openocd to use gdb from first terminal
openocd

# open gdb from second terminal
gdb-multiarch -x .gdbinit -q
```
