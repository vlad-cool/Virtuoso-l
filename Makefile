EXECS := send_btn send_rc5 get_pins
FLAGS := -lwiringPi -lm -lpthread -lcrypt -lrt

all: $(EXECS)

send_btn: send_btn.c
	gcc send_btn.c -o send_btn $(FLAGS)
	chmod 4711 send_btn

send_rc5: send_rc5.c
	gcc send_rc5.c -o send_rc5 $(FLAGS)
	chmod 4711 send_rc5

get_pins: get_pins.c
	gcc get_pins.c -o get_pins $(FLAGS)
	chmod 4711 get_pins

clean:
	rm $(EXECS)

clean_sources:
	rm $(addsuffix .c, $(EXECS))
	