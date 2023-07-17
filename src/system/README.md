# "System" directory
This provides functionality and abstractions for the kernel to use. 
This includes memory management, interrupts, and other low-level functionality.

## List of examples of things that should go here:
- Code that sets up the GDT, IDT, and other hardware tables
- Code that sets up paging
- Code that sets up interrupts, PIC, and similar
- Code that abstracts writing to VGA memory 
- Kernel panic handler

Please keep higher-level code out of this directory.

## Additional notes
- When handling a kernel panic, please do it in a way that introduces the least risk of another problem.
  - Rule 1: Avoid complex flow control, such as goto and recursion, in favor of simple flow control.
  - Rule 2: All loops must have fixed bounds, this will prevent infinite loops.
  - Rule 3: Avoid heap memory allocation, as this can cause memory leaks. 
    - Since you're already dealing with a panic, you don't want to risk another problem.
  - Rule 4: Restrict functions to a single printed page (around 60 lines)
  - Rule 5: Check the return value of all non-void functions.
- Feel free to use `unsafe` here, as this is the lowest level of the kernel, where hardware is being set up and managed.