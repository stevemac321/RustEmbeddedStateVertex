import subprocess
import os
import sys

def run_command(command):
    """Run a system command and ensure it completes successfully."""
    try:
        print(f"Running: {command}")
        result = subprocess.run(command, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        print(result.stdout.decode('utf-8'))
    except subprocess.CalledProcessError as e:
        print(f"Error occurred while running: {command}", file=sys.stderr)
        print(e.stderr.decode('utf-8'), file=sys.stderr)
        sys.exit(1)

def main():
    # Ensure we're in the project directory
    project_dir = os.path.dirname(os.path.realpath(__file__))
    
    # Define the path to the binary
    bin_path = os.path.join(project_dir, "target/thumbv7m-none-eabi/debug/rust_nucleo.bin")
    
    # Ensure the paths are correct when executing the commands
    objcopy_command = f"arm-none-eabi-objcopy -O binary ./target/thumbv7m-none-eabi/debug/rust_nucleo {bin_path}"
    erase_command = "st-flash erase"
    write_command = f"st-flash write {bin_path} 0x08000000"
    reset_command = "st-flash reset"

    # Run the commands sequentially, ensuring each one completes before starting the next
    run_command(objcopy_command)
    run_command(erase_command)
    run_command(write_command)
    run_command(reset_command)

if __name__ == "__main__":
    main()

