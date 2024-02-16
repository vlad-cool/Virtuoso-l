BANANA_IP := 192.168.50.23
DRIVER_EXECS := send_pin send_rc5 get_pin get_rc5

release: V24m_update.zip

V24m_update.zip: src/app.py src/system_info.py src/gpio_control.py src/video_control.py src/scripts/* src/main*.kv assets/*.TTF assets/*.png bin/* 
	rm -f release.zip
	mkdir -p app
	rm -rf app/*
	mkdir -p app/assets
	cp src/app.py src/system_info.py src/gpio_control.py src/video_control.py src/scripts/* src/main*.kv app/
	cp bin/* app/
	cp assets/*.TTF assets/*.png app/assets/
	cp -r assets/venv app/
	zip -r V24m_update.zip app

upload: release
	scp V24m_update.zip pi@$(BANANA_IP):
	ssh pi@$(BANANA_IP) "./install.sh"

clean:
	rm -rf release

remote_build: bin/get_pin bin/send_pin bin/get_rc5 bin/send_rc5
	ssh-add ~/.ssh/bananapi
	ssh -t pi@$(BANANA_IP) rm -rf gpio/*
	scp -r src/gpio pi@$(BANANA_IP):
	ssh -t pi@$(BANANA_IP) 'cd gpio && make && rm Makefile *.cpp *.hpp *.o'
	scp pi@$(BANANA_IP):gpio/* bin/
