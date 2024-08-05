#include <iostream>
#include <ctime>
#include <chrono>
#include <iomanip>

int main() {
    struct timespec boottime;
    clockid_t clock_id = CLOCK_BOOTTIME;

    if (clock_gettime(clock_id, &boottime) == 0) {
        std::cout << boottime.tv_sec << "." << std::setw(9) << std::setfill('0') << boottime.tv_nsec << std::endl;
    } else {
        std::cerr << "Failed to get boot time." << std::endl;
    }

    return 0;
}