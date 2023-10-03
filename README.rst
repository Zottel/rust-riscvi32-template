####################################################################
 Minimal template to build bare-metal RISC-V 32I binaries from Rust
####################################################################

This is a very minimal cargo project to demonstrate how to build bare-metal
binaries for an embedded platform.  It is intended as a starting point
from which processor tests or ROMs may be written.

While a surprising number of crates and tutorials will allow you to build
on this, it is not intended for application developers.  There is no HAL,
nor any infrastructure for drivers or flashing to platforms.


Getting Started
===============

To install rustup, follow the instructions at https://rustup.rs/

Install riscv32i compilation target::

  rustup target add riscv32i-unknown-none-elf

Build ELF file::

  cargo build --release

Install cargo-binutils::

  cargo install cargo-binutils
  rustup component add llvm-tools-preview

Convert compiled result to flat binary::

  cargo objcopy --release -- -O binary rust-riscv32i-template.bin

Disassemble produced binary with::

  cargo objdump --release -- --disassemble --no-show-raw-insn


TODO
====

Linker Base Offset
------------------

Currently, the linker script is set to use 0x8000 as the base address.

Is that a sensible default? Should we just set it to 0x0000?


References
==========

This repository was inspired by and builds on the bare metal Raspberry Pi
tutorials of the "Low Level Learning" Youtube channel:
https://www.youtube.com/@LowLevelLearning
https://www.youtube.com/watch?v=jZT8APrzvc4
https://www.youtube.com/watch?v=mPB3dCWlZVY

I spent some effort to use more of cargo and less command line parameters,
but those tutorials gave me a comfortable base to work on.


