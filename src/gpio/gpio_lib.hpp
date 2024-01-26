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

// gpiod::gpio

// void setup();

// void pinMode(int pin, int mode);
// void pinMode(const std::vector<int> pins, int mode);
// void pinMode(const std::vector<int> pins, const std::vector<int> mode);
// 
// void digitalWrite(int pin, int value);
// void pinMode(const std::vector<int> pins, int value);
// void pinMode(const std::vector<int> pins, const std::vector<int> values);

// int digitalRead(int pin);
// int digitalRead(const std::vector<int> pins);

int gpioSetupPhys();
int pinMode(int pin, int mode);
int digitalWrite(int pin, int value);
int digitalRead(int pin);
void gpioCleanup();

#endif