EXECS := send_pin send_rc5 get_pin get_rc5
EXECS := send_pin send_rc5 get_pin get_pin_extended get_rc5
FLAGS := -lwiringPi -lm -lpthread -lcrypt -lrt

all: $(EXECS)

%: %.c
	gcc $^ -o $@ $(FLAGS)
	chown root $@
	chmod 4755 $@

clean:
	rm $(EXECS)

clean_sources:
	rm $(addsuffix .c, $(EXECS))
	