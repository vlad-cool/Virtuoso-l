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
	ssh -t pi@$(BANANA_IP) rm -rf gpio/*
	scp -r src/gpio pi@192.168.2.12:
	ssh -t pi@$(BANANA_IP) 'cd gpio && make && rm Makefile *.cpp *.hpp *.o'
	scp pi@$(BANANA_IP):gpio/* bin/

local_debug:
	mkdir -p local_debug
	rm -rf local_debug/*

	ln -s src/app.py local_debug/app.py
	ln -s src/gpio_control_emu.py local_debug/gpio_control.py
	ln -s src/video_control_emu.py local_debug/video_control.py

	