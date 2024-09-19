CC := g++
CFLAGS := -c -O3
LDFLAGS := -lgpiodcxx

SRCS := $(wildcard *.cpp)
OBJS := $(patsubst %.cpp,%.o,$(SRCS))
TARGETS := send_pin send_rc5 get_pin get_rc5

all: $(TARGETS)

%.o: %.cpp gpio_lib.hpp
	$(CC) $(CFLAGS) $< -o $@

$(TARGETS): $(OBJS) gpio_lib.o
	$(CC) $@.o gpio_lib.o -o $@ $(LDFLAGS)

clean:
	rm -f $(TARGETS) $(OBJS)