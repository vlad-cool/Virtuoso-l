#include "gpio_lib.hpp"
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <vector>
#include <map>

#define TIMING 200000

std::map<int, int> pins;
std::vector<int> static_pins{8, 10, 12, 16, 19, 21, 23, 24};

void flush()
{
    while (getchar() != '\n');
    printf("Not a number\n");
}

void setup()
{
    gpioSetupPhys();
}

void set(int pin, int val)
{
    auto pin_it = pins.find(pin);
    if (pin_it != pins.end()) {
        pins[pin] = val;
        digitalWrite(pin, pins[pin]);
    }
    else
    {
        std::cerr << "Unknown pin" << std::endl;
    }
}

void toggle(int pin)
{
    auto pin_it = pins.find(pin);
    if (pin_it != pins.end()) {
        pins[pin] = 1 - pins[pin];
        digitalWrite(pin, 1 - pins[pin]);
    }
    else
    {
        std::cerr << "Unknown pin" << std::endl;
    }
}

void button(int pin)
{
    toggle(pin);
    usleep(TIMING);
    toggle(pin);
    usleep(TIMING);
}

int main()
{
    int pin = 0, value = 0;
    char s[128];

    while (1)
    {
        if (scanf("%127s", s) < 1)
        {
            break;
        }
        if (strcmp(s, "add_pin") == 0)
        {
            if (scanf("%d %d", &pin, &value) < 2)
                flush();
            else
            {
                // digitalWrite(pin, value);
                pins[pin] = value;
            }
            continue;
        }
        if (strcmp(s, "setup") == 0)
        {
            setup();
            for (auto pin = static_pins.begin(); pin != static_pins.end(); pin++)
            {
                pinMode(*pin, 1);
                digitalWrite(*pin, 1);
            }
            for (auto pin = pins.begin(); pin != pins.end(); pin++)
            {
                pinMode(pin->first, 1);
                digitalWrite(pin->first, pin->second);
            }
            // sleep(1);
            continue;
        }
        if (strcmp(s, "toggle") == 0)
        {
            if (scanf("%d", &pin) < 1)
                flush();
            else
                toggle(pin);
            continue;
        }
        if (strcmp(s, "button") == 0)
        {
            if (scanf("%d", &pin) < 1)
                flush();
            else
                button(pin);
            continue;
        }
        if (strcmp(s, "set") == 0)
        {
            if (scanf("%d %d", &pin, &value) < 2)
                flush();
            else
                set(pin, value);
            continue;
        }
        if (strcmp(s, "exit") == 0)
        {
            break;
        }
    }
}