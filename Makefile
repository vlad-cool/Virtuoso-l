BANANA_IP := 192.168.2.12
DRIVER_EXECS := send_pin send_rc5 get_pin get_rc5

release:
	mkdir -p release
	rm -rf release/*
	cp src/app.py src/gpio_control.py src/gpio_control_emu.py src/video_control.py src/video_control_emu.py release/
	cp bin/* release/

clean:
	rm -rf release

remote_build:
	ssh-add ~/.ssh/bananapi
	ssh -t pi@$(BANANA_IP) mkdir -p src
	ssh -t pi@$(BANANA_IP) rm -rf src/*
	scp src/*.c pi@$(BANANA_IP):src/
	scp src/Makefile_gpio pi@$(BANANA_IP):src/Makefile
	ssh -t pi@$(BANANA_IP) 'cd src && sudo make'
	ssh -t pi@$(BANANA_IP) rm src/*.c src/Makefile
	scp pi@$(BANANA_IP):src/* bin/

remote_install_drivers:
	ssh -t pi@$(BANANA_IP) 'cd V24m && sudo rm $(DRIVER_EXECS)'
	scp bin/* pi@$(BANANA_IP):V24m/
	ssh -t pi@$(BANANA_IP) 'cd V24m && sudo chown root $(DRIVER_EXECS)'
	ssh -t pi@$(BANANA_IP) 'cd V24m && sudo chmod 4755 $(DRIVER_EXECS)'
