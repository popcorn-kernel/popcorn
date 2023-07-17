# "Architecture" directory
This directory contains the necessary .JSON configuration files to build the kernel for a specific architecture. This includes the linker script, the target triple, and other things.

## List of examples of things that should go here:
- JSON for x86-64 / AMD64 (Ordinary 64-bit x86)
- JSON for i686 (Ordinary 32-bit x86)
- JSON for ARM
- JSON for RISC-V
- JSON for MIPS... You get the idea. Need I say more?

## List of examples of things that should NOT go here:
- Code. Please use [the system directory](../src/kernel/README.md) for architecture-specific code.

## Example of a valid .JSON architecture file:
```json
{
  "llvm-target": "x86_64-unknown-none",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "arch": "x86_64",
  "target-endian": "little",
  "target-pointer-width": "64",
  "target-c-int-width": "32",
  "os": "none",
  "executables": true,
  "linker-flavor": "ld.lld",
  "linker": "rust-lld",
  "panic-strategy": "abort",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float"
}
```
### Explanation of the above JSON:
- `llvm-target`: The LLVM target triple. This is used by the Rust compiler to determine what toolchain to use for building.
- `data-layout`: The data layout. This is used by the Rust compiler to determine how to lay out data in memory.
- `arch`: The architecture. This is used to determine what CPU architecture to build for.
    - Examples: `x86_64`, etc. Untested to work with anything other than x86_64.
- `target-endian`: The endianness of the target architecture. This is used to determine how bits are ordered in memory.
    - Examples: `little`, `big`. Little-endian is the most common.
- `target-pointer-width`: The pointer width of the target architecture. This is used to determine how large pointers are.
    - Examples: `32`, `64`. 64-bit is the most common, and allows for more memory to be addressed.
- `target-c-int-width`: The C integer width of the target architecture. This is used to determine how large C integers are.
    - Examples: `32`, `64`. 64-bit is the most common, and allows for more memory to be addressed.
- `os`: The operating system. This is used to determine what operating system to build for.
    - Examples: `none`, `linux`, `windows`, `macos`, etc. ***Keep this at `none` at the moment!***
- `executables`: Whether or not to build executables. This is used to determine whether or not to build executables.
    - Examples: `true`, `false`.
- `linker-flavor`: The linker flavor. This is used to determine what linker to use.
- `linker`: The linker. This is used to determine what linker to use.
- `panic-strategy`: The panic strategy. This is used to determine what to do when a panic occurs.
    - Examples: `abort`, `unwind`.
- `disable-redzone`: Whether or not to disable the redzone, which is a small amount of memory that is reserved for the stack.
    - Examples: `true`, `false`.
- `features`: The features of the instruction set. You can add a feature and use it in your code.
    - Examples: `-mmx,-sse,+soft-float`.