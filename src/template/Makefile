all: ../main1920x360.kv ../main1920x480.kv ../main1920x550.kv ../main1920x360_no_video.kv

../main1920x360.kv: template_main.kv 1920x360.json colors.json build.py video_info.kv video_player.kv
	./build.py 1920x360.json 1920x360_gen.json
	mv output.kv ../main1920x360.kv

../main1920x480.kv: template_main.kv 1920x480.json colors.json build.py video_info.kv video_player.kv
	./build.py 1920x480.json 1920x480_gen.json
	mv output.kv ../main1920x480.kv

../main1920x550.kv: template_main.kv 1920x550.json colors.json build.py video_info.kv video_player.kv
	./build.py 1920x550.json 1920x550_gen.json
	mv output.kv ../main1920x550.kv

../main1920x360_no_video.kv: template_main.kv 1920x360_no_video.json colors.json build.py video_info.kv video_player.kv
	./build.py 1920x360_no_video.json 1920x360_gen.json
	mv output.kv ../main1920x360_no_video.kv
