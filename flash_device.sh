#!/bin/sh

cargo build

target="target/thumbv7em-none-eabihf/debug/fs-ia6b-nucleo"

sudo openocd -f openocd.cfg -c "program ${target} verify reset exit"
