# all: ../main1920x360.kv ../main1920x480.kv ../main1920x550.kv
all: ../main1920x480.kv ../main1920x360.kv

../main1920x360.kv: template_main.kv 1920x360.json
	./build.py 1920x360.json
	mv output.kv ../main1920x360.kv

../main1920x480.kv: template_main.kv 1920x480.json
	./build.py 1920x480.json
	mv output.kv ../main1920x480.kv

../main1920x550.kv: template_main.kv 1920x550.json
	./build.py 1920x550.json
	mv output.kv ../main1920x550.kv
