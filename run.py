import os
import subprocess
import time

# Define paths to necessary files
project_dir = os.path.dirname(os.path.realpath(__file__))
elf_path = os.path.join(project_dir, "target/thumbv7m-none-eabi/debug/rust_nucleo")
gdb_script = os.path.join(project_dir, "gdb_commands.gdb")  # Your GDB commands script

# Start OpenOCD
openocd_process = subprocess.Popen([
    'openocd', 
    '-f', 'interface/stlink.cfg',  # Update to match your OpenOCD config
    '-f', 'target/stm32f4x.cfg',   # Update to match your MCU
    '-c', 'init', 
    '-c', 'reset init'
])

# Give OpenOCD some time to initialize
time.sleep(2)

# Start GDB-Multiarch in quiet mode
gdb_process = subprocess.Popen([
    'gdb-multiarch',
    '-q',                # Quiet mode (disable unnecessary output)
    '--command=' + gdb_script,  # Your GDB script file
    elf_path             # ELF file with symbols
])

# Wait for GDB to finish
gdb_process.wait()

# Kill the OpenOCD process after GDB finishes
openocd_process.terminate()
