kernel_image="kernel8.img"
kenrnel_dtb="linux_raspberrypi/build_4b/arch/arm64/boot/dts/broadcom/bcm2710-rpi-3-b-plus.dtb"
IMG="2024-03-12-raspios-bookworm-arm64-lite.img"
qemu-system-aarch64 \
        -smp 4 -cpu cortex-a72 \
        -machine raspi4b \
        -m 2G \
        -k en-us \
        -dtb $kenrnel_dtb \
        -kernel $kernel_image \
        -drive id=hd-root,format=raw,file=$IMG \
        -append "rw earlycon=pl011,0x3f201000 console=ttyAMA0 loglevel=8 root=/dev/mmcblk0p2 \
        fsck.repair=yes net.ifnames=0 rootwait memtest=1 dwc_otg.fiq_fsm_enable=1" \
        -serial stdio \
        -usb -device usb-kbd \
        -monitor telnet:127.0.0.1:5555,server,nowait \
        -device usb-tablet -device usb-net \
        -nographic \
        -vnc :1
