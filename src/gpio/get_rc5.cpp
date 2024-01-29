#include "gpio_lib.hpp"
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <sys/poll.h>

#define TIMING 444
#define rc5_pin 3

int buffer[56] = {0};

void setup()
{
    gpioSetupPhys();
    pinMode(rc5_pin, 0);
}

int main()
{
    setup();
    char s[128];
    struct timespec t;
    unsigned long time, timer;
    int toggle = -1;

    while (1)
    {
        struct pollfd fds;
        int ret;
        fds.fd = 0;
        fds.events = POLLIN;
        ret = poll(&fds, 1, 0);
        if(ret == 1)
        {
            if (scanf("%127s", s) < 1)
            {
                break;
            }
            if (strcmp(s, "get") == 0)
            {
                printf("end\n");
                fflush(stdout);
            }
            if (strcmp(s, "exit") == 0)
            {
                break;
            }
        }

        clock_gettime(CLOCK_BOOTTIME, &t);
        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;

        for (int i = 0; i < 55; i++)
        {
            buffer[i] = buffer[i + 1];
        }
        buffer[55] = digitalRead(rc5_pin);

        if (buffer[1] == 1 && buffer[3] == 0 && buffer[5] == 1 && buffer[7] == 0)
        {
            int valid = 1;
            for (int i = 0; i < 14; i++)
            {
                if (buffer[i * 4 + 1] + buffer[i * 4 + 3] != 1)
                {
                    valid = 0;
                }
            }

            if (valid)
            {
                toggle = buffer[9];
                for (int i = 0; i < 28; i++)
                {
                    printf(" %d", buffer[i * 2 + 1]);
                }
                printf("\n");
                fflush(stdout);
                for (int i = 0; i < 56; i++)
                {
                    buffer[i] = 0;
                }
            }
        }

        usleep(TIMING - 100);

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }
    }
}