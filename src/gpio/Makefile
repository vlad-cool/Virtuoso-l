all: send_pin send_rc5 get_pin get_rc5

gpio_lib.o: gpio_lib.cpp gpio_lib.hpp
	g++ -c gpio_lib.cpp

send_pin.o: send_pin.cpp gpio_lib.hpp
	g++ -c send_pin.cpp

send_rc5.o: send_rc5.cpp gpio_lib.hpp
	g++ -c send_rc5.cpp

get_pin.o: get_pin.cpp gpio_lib.hpp
	g++ -c get_pin.cpp

get_rc5.o: get_rc5.cpp gpio_lib.hpp
	g++ -c get_rc5.cpp

send_pin: send_pin.o gpio_lib.o
	g++ send_pin.o gpio_lib.o -o send_pin -lgpiodcxx

send_rc5: send_rc5.o gpio_lib.o
	g++ send_rc5.o gpio_lib.o -o send_rc5 -lgpiodcxx

get_pin: get_pin.o gpio_lib.o
	g++ get_pin.o gpio_lib.o -o get_pin -lgpiodcxx

get_rc5: get_rc5.o gpio_lib.o
	g++ get_rc5.o gpio_lib.o -o get_rc5 -lgpiodcxx

clean:
	rm send_pin send_rc5 get_pin get_rc5 *.o

clean_sources:
	rm Makefile *.cpp *.hpp