sudo mkdir -p /usr/local/lib/firmware
sudo cp bootsplash.armbian /usr/local/lib/firmware

splashfile=/usr/local/lib/firmware/bootsplash.armbian

if [ -f "${splashfile}" ]; then
    sudo cp "${splashfile}" "${DESTDIR}"/lib/firmware
fi

sudo update-initramfs -v -u
