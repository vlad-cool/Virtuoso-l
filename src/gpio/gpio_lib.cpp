#include "gpio_lib.hpp"

std::vector<::gpiod::chip> chips;

std::map<int, std::pair<int, int>> pin_map;

int gpioSetupPhys() {
    chips.push_back(::gpiod::chip("/dev/gpiochip0"));
    chips.push_back(::gpiod::chip("/dev/gpiochip1"));
    for (auto iter = chips.begin(); iter != chips.end(); iter++)
    {
        auto chip = *iter;
        if (!chip) {
            std::cerr << "Failed to open GPIO chip" << std::endl;
            return -1;
        }
    }

    // Magic numbers below got from gpioinfo
    pin_map = std::map<int, std::pair<int, int>>({
        {13, {0,  0}},
        {11, {0,  1}},
        {22, {0,  2}},
        {15, {0,  3}},
        { 7, {0,  6}},
        {29, {0,  7}},
        {31, {0,  8}},
        {33, {0,  9}},
        {35, {0, 10}},
        { 5, {0, 11}},
        { 3, {0, 12}},
        { 8, {0, 13}},
        {10, {0, 14}},
        {16, {0, 15}},
        {12, {0, 16}},
        {37, {0, 17}},
        {28, {0, 18}},
        {27, {0, 19}},
        {40, {0, 20}},
        {38, {0, 21}},
        {19, {0, 64}},
        {21, {0, 65}},
        {23, {0, 66}},
        {24, {0, 67}},
        {18, {0, 68}},
        {26, {0, 71}},
        {32, {1,  2}},
        {36, {1,  4}},
    });

    return 0;
}

int pinMode(int pin, int mode) {
    auto line = chips[pin_map[pin].first].get_line(pin_map[pin].second);
    if (!line) {
        std::cerr << "Failed to get GPIO " << pin << " line" << std::endl;
        return -1;
    }

    switch (mode) {
        case INPUT:
            try {
                line.request({"libgpiod-wrapper", gpiod::line_request::DIRECTION_INPUT});
            } catch (const std::exception& e) {
                std::cerr << "Failed to set GPIO " << pin << " line as input: " << e.what() << std::endl;
                return -1;
            }
            break;
        case OUTPUT:
            try {
                line.request({"libgpiod-wrapper", gpiod::line_request::DIRECTION_OUTPUT, 0});
            } catch (const std::exception& e) {
                std::cerr << "Failed to set GPIO " << pin << " line as output: " << e.what() << std::endl;
                return -1;
            }
            break;
        default:
            std::cerr << "Invalid pin mode" << std::endl;
            return -1;
            break;
    }

    return 0;
}

int digitalWrite(int pin, int value) {
    auto line = chips[pin_map[pin].first].get_line(pin_map[pin].second);
    if (!line) {
        std::cerr << "Failed to get GPIO " << pin << " line" << std::endl;
        return -1;
    }

    try {
        line.set_value(value);
    } catch (const std::exception& e) {
        std::cerr << "Failed to set GPIO " << pin << " value: " << e.what() << std::endl;
        return -1;
    }

    return 0;
}

int digitalRead(int pin) {
    auto line = chips[pin_map[pin].first].get_line(pin_map[pin].second);
    if (!line) {
        std::cerr << "Failed to get GPIO " << pin << " line" << std::endl;
        return -1;
    }

    try {
        return line.get_value();
    } catch (const std::exception& e) {
        std::cerr << "Failed to read GPIO " << pin << " value: " << e.what() << std::endl;
        return -1;
    }
}

void gpioCleanup() {
    for (auto iter = chips.begin(); iter != chips.end(); iter++)
    {
        iter->reset();
    }
}
