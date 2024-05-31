cd arceos
make clean
make A=apps/cli ARCH=aarch64 PLATFORM=aarch64-raspi4 LOG=debug
cd ../rust-raspberrypi-OS-tutorials/06_uart_chainloader
make clean
make BSP=rpi4
cd ../..
# cat rust-raspberrypi-OS-tutorials/06_uart_chainloader/kernel8.img arceos/apps/boards/raspi4/raspi4_aarch64-raspi4.bin > kernel8.img
cat rust-raspberrypi-OS-tutorials/06_uart_chainloader/kernel8.img arceos/apps/cli/cli_aarch64-raspi4.bin > kernel8.img
