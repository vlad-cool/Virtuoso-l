BANANA_IP := 192.168.2.12
DRIVER_EXECS := send_pin send_rc5 get_pin get_rc5

release:
	mkdir -p release_dir
	rm -rf release_dir/*
	cp src/app.py src/gpio_control.py src/video_control.py src/scripts/* release_dir/
	cp bin/* release_dir/

clean:
	rm -rf release

remote_build:
	ssh-add ~/.ssh/bananapi
	ssh -t pi@$(BANANA_IP) rm -rf gpio/*
	scp -r src/gpio pi@192.168.2.12:
	ssh -t pi@$(BANANA_IP) 'cd gpio && make && rm Makefile *.cpp *.hpp *.o'
	scp pi@$(BANANA_IP):gpio/* bin/
