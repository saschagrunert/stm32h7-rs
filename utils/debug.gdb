target ext :3333

set remote hardware-breakpoint-limit 8
set remote hardware-watchpoint-limit 4

# print demangled symbols by default
set print asm-demangle on

# Reset to known state
monitor arm semihosting enable
monitor reset halt

# Load the program executable
load
step
