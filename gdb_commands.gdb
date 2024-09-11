tar extended-remote :3333
set confirm off
monitor arm semihosting enable
break dummy  # Set breakpoint at dummy
info breakpoints  # Check if the breakpoint was set correctly
continue  # Continue until breakpoint is hit
# Optional: You can add `step` here if you need to step through instructions
kill  # Kill the process after hitting the breakpoint
quit
