KERNELDIR ?= /home/oveln/projects/R4L_DRV/linux_raspberrypi
PWD := $(shell pwd)

obj-m := c_print.o

all:
	make -C $(KERNELDIR) M=$(PWD) modules

clean:
	rm -rf *.o *~ core .depend .*.cmd *.ko *.mod.c .tmp_versions *.mod *.order *.symvers
