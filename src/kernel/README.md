# "Kernel" directory
This directory contains the main kernel code. This utilizes the features provided in the `system` directory.

## Kernel entry point
The kernel entry point should be located in `src/kernel/kernel_main.rs`. This is where the kernel should be initialized.

## List of examples of things that should go here:
- The scheduler
- System call interrupt handler and related

## List of examples of things that should NOT go here:
- Code directly managing hardware, memory, interrupts, etc
- Userland code and Drivers
- Code that is not related to the kernel