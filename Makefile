BANANA_IP := 192.168.2.12
DRIVER_EXECS := send_pin send_rc5 get_pin get_rc5

.PHONY: release ssh upload clean

release: Virtuoso_update.zip 

layouts: src/template/1920x360_gen.json src/template/1920x480_gen.json src/template/1920x550_gen.json

src/template/1920x360_gen.json: scripts/parse_svg.py prototypes/1920x360.svg
	python scripts/parse_svg.py prototypes/1920x360.svg src/template/1920x360_gen.json

src/template/1920x480_gen.json: scripts/parse_svg.py prototypes/1920x480.svg
	python scripts/parse_svg.py prototypes/1920x480.svg src/template/1920x480_gen.json

src/template/1920x550_gen.json: scripts/parse_svg.py prototypes/1920x550.svg
	python scripts/parse_svg.py prototypes/1920x550.svg src/template/1920x550_gen.json

ssh:
	ssh-add ~/.ssh/bananapi

Virtuoso_update.zip: src/* assets/* bin/* src/*/* VERSION
	cd src/template && make
	rm -f Virtuoso_update.zip
	mkdir -p app
	rm -rf app/*
	mkdir -p app/assets
	cp src/app.py src/system_info.py src/gpio_control.py src/video_control.py src/static_vars.py src/get_comment_metadata.sh src/recorder.sh src/scripts/* src/main*.kv app/
	cp bin/* app/
	cp assets/*.TTF assets/*.png app/assets/
	cp -r assets/venv app/
	cp VERSION app/
	zip -r Virtuoso_update.zip app

upload: release ssh
	scp Virtuoso_update.zip pi@$(BANANA_IP):Virtuoso/
	ssh pi@$(BANANA_IP) ./install.sh
	ssh pi@$(BANANA_IP) ./kill.sh

clean:
	rm -rf release

remote_build: bin/gpio/*

bin/gpio/*: src/gpio/* ssh
	ssh -t pi@$(BANANA_IP) rm -rf gpio/*
	scp -r src/gpio pi@$(BANANA_IP):
	ssh -t pi@$(BANANA_IP) 'cd gpio && make'
	scp pi@$(BANANA_IP):gpio/get_pin pi@$(BANANA_IP):gpio/get_rc5 pi@$(BANANA_IP):gpio/send_pin pi@$(BANANA_IP):gpio/send_rc5 pi@$(BANANA_IP):gpio/get_time bin/
