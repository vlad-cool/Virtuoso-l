#ifndef GPIO_LIB
#define GPIO_LIB

#include <map>
#include <vector>
#include <iostream>
#include <gpiod.hpp>

enum modes {
    INPUT,
    OUTPUT,
};

// TODO add vectors support

int gpioSetupPhys();
int pinMode(int pin, int mode);
int digitalWrite(int pin, int value);
int digitalRead(int pin);
void gpioCleanup();

#endif