#include "gpio_lib.hpp"

std::vector<::gpiod::chip> chips;

std::pair<int, int> pin_map[40] = {};

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
    pin_map[13] = {0,  0};
    pin_map[11] = {0,  1};
    pin_map[22] = {0,  2};
    pin_map[15] = {0,  3};
    pin_map[ 7] = {0,  6};
    pin_map[29] = {0,  7};
    pin_map[31] = {0,  8};
    pin_map[33] = {0,  9};
    pin_map[35] = {0, 10};
    pin_map[ 5] = {0, 11};
    pin_map[ 3] = {0, 12};
    pin_map[ 8] = {0, 13};
    pin_map[10] = {0, 14};
    pin_map[16] = {0, 15};
    pin_map[12] = {0, 16};
    pin_map[37] = {0, 17};
    pin_map[28] = {0, 18};
    pin_map[27] = {0, 19};
    pin_map[40] = {0, 20};
    pin_map[38] = {0, 21};
    pin_map[19] = {0, 64};
    pin_map[21] = {0, 65};
    pin_map[23] = {0, 66};
    pin_map[24] = {0, 67};
    pin_map[18] = {0, 68};
    pin_map[26] = {0, 71};
    pin_map[32] = {1,  2};
    pin_map[36] = {1,  4};

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
