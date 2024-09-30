RESOLUTIONS := 1920x360_notouch 1920x480_notouch 1920x550_notouch

.PHONY: Virtuoso_1920x360_notouch.img Virtuoso_1920x480_notouch.img Virtuoso_1920x550_notouch.img UMOUNT

all: Virtuoso_1920x360_notouch.img Virtuoso_1920x480_notouch.img Virtuoso_1920x550_notouch.img 

UMOUNT:
	-umount mnt

Virtuoso_1920x360_notouch.img: UMOUNT
	cp --preserve=ownership base_image.img Virtuoso_1920x360_notouch.img

	mount -o loop,rw,sync,offset=8594128896 Virtuoso_1920x360_notouch.img mnt
	cp ../Virtuoso_update.zip mnt/
	cd mnt && unzip Virtuoso_update.zip
	rm mnt/Virtuoso_update.zip
	umount mnt

	mount -o loop,rw,sync,offset=4194304 Virtuoso_1920x360_notouch.img mnt
	cp --preserve=ownership src_1920x360_notouch/50-Virtuoso.conf mnt/etc/X11/xorg.conf.d/
	cp --preserve=ownership src_1920x360_notouch/armbianEnv.txt mnt/boot/
	cp --preserve=ownership src_1920x360_notouch/config.ini mnt/home/pi/.kivy/
	cp --preserve=ownership src_1920x360_notouch/config.sh mnt/home/pi/
	cp --preserve=ownership -r src_1920x360_notouch/Virtuoso_splash1920x360 mnt/usr/share/plymouth/themes/
	umount mnt

	image_exec -i Virtuoso_1920x360_notouch.img -p 1 -u "update-alternatives --install /usr/share/plymouth/themes/default.plymouth default.plymouth /usr/share/plymouth/themes/Virtuoso_splash1920x360/Virtuoso_splash1920x360.plymouth 220"
	image_exec -i Virtuoso_1920x360_notouch.img -p 1 -u "update-initramfs -v -u"

Virtuoso_1920x480_notouch.img: UMOUNT
	cp --preserve=ownership base_image.img Virtuoso_1920x480_notouch.img

	mount -o loop,rw,sync,offset=8594128896 Virtuoso_1920x480_notouch.img mnt
	cp ../Virtuoso_update.zip mnt/
	cd mnt && unzip Virtuoso_update.zip
	rm mnt/Virtuoso_update.zip
	umount mnt

	mount -o loop,rw,sync,offset=4194304 Virtuoso_1920x480_notouch.img mnt
	cp --preserve=ownership src_1920x480_notouch/50-Virtuoso.conf mnt/etc/X11/xorg.conf.d/
	cp --preserve=ownership src_1920x480_notouch/armbianEnv.txt mnt/boot/
	cp --preserve=ownership src_1920x480_notouch/config.ini mnt/home/pi/.kivy/
	cp --preserve=ownership src_1920x480_notouch/config.sh mnt/home/pi/
	cp --preserve=ownership -r src_1920x480_notouch/Virtuoso_splash1920x480 mnt/usr/share/plymouth/themes/
	umount mnt

	image_exec -i Virtuoso_1920x480_notouch.img -p 1 -u "update-alternatives --install /usr/share/plymouth/themes/default.plymouth default.plymouth /usr/share/plymouth/themes/Virtuoso_splash1920x480/Virtuoso_splash1920x480.plymouth 220"
	image_exec -i Virtuoso_1920x480_notouch.img -p 1 -u "update-initramfs -v -u"

Virtuoso_1920x550_notouch.img: UMOUNT
	cp --preserve=ownership base_image.img Virtuoso_1920x550_notouch.img

	mount -o loop,rw,sync,offset=8594128896 Virtuoso_1920x550_notouch.img mnt
	cp ../Virtuoso_update.zip mnt/
	cd mnt && unzip Virtuoso_update.zip
	rm mnt/Virtuoso_update.zip
	umount mnt

	mount -o loop,rw,sync,offset=4194304 Virtuoso_1920x550_notouch.img mnt
	cp --preserve=ownership src_1920x550_notouch/50-Virtuoso.conf mnt/etc/X11/xorg.conf.d/
	cp --preserve=ownership src_1920x550_notouch/armbianEnv.txt mnt/boot/
	cp --preserve=ownership src_1920x550_notouch/config.ini mnt/home/pi/.kivy/
	cp --preserve=ownership src_1920x550_notouch/config.sh mnt/home/pi/
	cp --preserve=ownership -r src_1920x550_notouch/Virtuoso_splash1920x550 mnt/usr/share/plymouth/themes/
	umount mnt

	image_exec -i Virtuoso_1920x550_notouch.img -p 1 -u "update-alternatives --install /usr/share/plymouth/themes/default.plymouth default.plymouth /usr/share/plymouth/themes/Virtuoso_splash1920x550/Virtuoso_splash1920x550.plymouth 220"
	image_exec -i Virtuoso_1920x550_notouch.img -p 1 -u "update-initramfs -v -u"


Virtuoso_1920x360_notouch_novideo.img: UMOUNT
	cp --preserve=ownership base_image.img Virtuoso_1920x360_notouch_novideo.img

	mount -o loop,rw,sync,offset=8594128896 Virtuoso_1920x360_notouch_novideo.img mnt
	cp ../Virtuoso_update.zip mnt/
	cd mnt && unzip Virtuoso_update.zip
	rm mnt/Virtuoso_update.zip
	umount mnt

	mount -o loop,rw,sync,offset=4194304 Virtuoso_1920x360_notouch_novideo.img mnt
	cp --preserve=ownership src_1920x360_notouch_novideo/50-Virtuoso.conf mnt/etc/X11/xorg.conf.d/
	cp --preserve=ownership src_1920x360_notouch_novideo/armbianEnv.txt mnt/boot/
	cp --preserve=ownership src_1920x360_notouch_novideo/config.ini mnt/home/pi/.kivy/
	cp --preserve=ownership src_1920x360_notouch_novideo/config.sh mnt/home/pi/
	cp --preserve=ownership -r src_1920x360_notouch_novideo/Virtuoso_splash1920x360 mnt/usr/share/plymouth/themes/
	umount mnt

	image_exec -i Virtuoso_1920x360_notouch_novideo.img -p 1 -u "update-alternatives --install /usr/share/plymouth/themes/default.plymouth default.plymouth /usr/share/plymouth/themes/Virtuoso_splash1920x360/Virtuoso_splash1920x360.plymouth 220"
	image_exec -i Virtuoso_1920x360_notouch_novideo.img -p 1 -u "update-initramfs -v -u"
