# Setup and Running

This is the QEMU branch where you don't need any hardware. You will need to install the same tools from the main branch and install QEMU.

```
# Install QEMU.
sudo apt install qemu-system-arm

# Run QEMU.
cargo run --release --target thumbv6m-none-eabi
```
