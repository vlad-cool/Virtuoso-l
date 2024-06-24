BANANA_IP := 192.168.2.9
DRIVER_EXECS := send_pin send_rc5 get_pin get_rc5

release: V24m_update.zip

ssh:
	ssh-add ~/.ssh/bananapi

V24m_update.zip: src/* assets/* bin/*
	cd src/template && make
	rm -f V24m_update.zip
	mkdir -p app
	rm -rf app/*
	mkdir -p app/assets
	cp src/app.py src/system_info.py src/gpio_control.py src/video_control.py src/static_vars.py src/get_comment_metadata.sh src/scripts/* src/main*.kv app/
	cp bin/* app/
	cp assets/*.TTF assets/*.png app/assets/
	cp -r assets/venv app/
	cp VERSION app/
	zip -r V24m_update.zip app

upload: release ssh
	scp V24m_update.zip pi@$(BANANA_IP):V24m/
	ssh pi@$(BANANA_IP) ./install.sh
	ssh pi@$(BANANA_IP) ./kill.sh

clean:
	rm -rf release

remote_build: bin/gpio/*

bin/gpio/*: src/gpio/* ssh
	ssh-add ~/.ssh/bananapi
	ssh -t pi@$(BANANA_IP) rm -rf gpio/*
	scp -r src/gpio pi@$(BANANA_IP):
	ssh -t pi@$(BANANA_IP) 'cd gpio && make'
	scp pi@$(BANANA_IP):gpio/get_pin pi@$(BANANA_IP):gpio/get_rc5 pi@$(BANANA_IP):gpio/send_pin pi@$(BANANA_IP):gpio/send_rc5 bin/
