file target/thumbv6m-none-eabi/debug/nrf51822_blinky_rust
set remotetimeout 60000
target remote:3333
monitor reset halt
break main
continue
layout src
