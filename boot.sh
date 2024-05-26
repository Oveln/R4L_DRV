kernel_image="linux_raspberrypi/build_4b/arch/arm64/boot/Image"
kenrnel_dtb="linux_raspberrypi/build_4b/arch/arm64/boot/dts/broadcom/bcm2710-rpi-3-b-plus.dtb"
IMG="2024-03-12-raspios-bookworm-arm64-lite.img"
qemu-system-aarch64 \
        -machine type=raspi3b \
        -m 1G \
        -k en-us \
        -dtb $kenrnel_dtb \
        -kernel $kernel_image \
        -drive id=hd-root,format=raw,file=$IMG \
        -append "rw earlycon=pl011,0x3f201000 console=ttyAMA0 loglevel=8 root=/dev/mmcblk0p2 \
        fsck.repair=yes net.ifnames=0 rootwait memtest=1 dwc_otg.fiq_fsm_enable=0" \
        -serial stdio \
        -usb -device usb-kbd \
        -netdev user,id=net0,hostfwd=tcp::2222-:22 \
        -device usb-net,netdev=net0 \
        -monitor telnet:127.0.0.1:5555,server,nowait \
        -device usb-tablet \
        -nographic
