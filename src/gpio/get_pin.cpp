#include "gpio_lib.hpp"
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <sys/poll.h>
#include <vector>

std::vector<int> pins;

int pin_18_state_buf[8] = {0};

void flush()
{
    while (getchar() != '\n');
}

void setup()
{
    gpioSetupPhys();
    pinMode(18, 0);
}

int main()
{
    setup();
    char s[128];
    struct timespec t;
    long long time = 0;
    int pin;

    while (1)
    {
        struct pollfd fds;
        int ret;
        fds.fd = 0;
        fds.events = POLLIN;
        ret = poll(&fds, 1, 5);
        if(ret == 1)
        {
            if (scanf("%127s", s) < 1)
            {
                break;
            }
            if (strcmp(s, "add_pin") == 0)
            {
                if (scanf("%d", &pin) < 1)
                    flush();
                else
                {
                    pinMode(pin, 0);
                    pins.push_back(pin);
                }
                continue;
            }
            if (strcmp(s, "get") == 0)
            {
                printf("{");
                for (auto pin = pins.begin(); pin != pins.end(); pin++)
                {
                    printf("%d: %d, ", *pin, digitalRead(*pin));
                }

                int pin_18_state = 0;
                for (int i = 0; i < sizeof(pin_18_state_buf) / sizeof(int); i++)
                {
                    pin_18_state += pin_18_state_buf[i];
                }
                printf("%d: %d, ", 18, pin_18_state == 0 ? 0 : 1);

                printf("}\n");
                fflush(stdout);
            }
            if (strcmp(s, "exit") == 0)
            {
                break;
            }
        }

        int i;

        for (i = 0; i + 1 < sizeof(pin_18_state_buf) / sizeof(int); i++)
        {
            pin_18_state_buf[i] = pin_18_state_buf[i + 1];
        }
        pin_18_state_buf[i] = digitalRead(18);
    }
}