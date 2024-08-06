#include "gpio_lib.hpp"
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <sys/poll.h>

#define TIMING 889
#define rc5_pin 3

int buffer[28] = {1};

void setup()
{
    gpioSetupPhys();
    pinMode(rc5_pin, 0);
}

int main()
{
    setup();
    char s[128];
    struct timespec t_start, t_end;

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

        buffer[1] = digitalRead(rc5_pin);

        if (buffer[1] == 1)
        {
            usleep(TIMING / 2);
        }
        else
        {
            clock_gettime(CLOCK_BOOTTIME, &t_end);
            for (int i = 1; i < 28; i++)
            {
                t_start.tv_sec = t_end.tv_sec;
                t_start.tv_nsec = t_end.tv_nsec;
                buffer[i] = digitalRead(rc5_pin);
                do {
                    clock_gettime(CLOCK_BOOTTIME, &t_end);
                } while ((t_end.tv_sec - t_start.tv_sec) * 1000 * 1000 + (t_end.tv_nsec - t_start.tv_nsec) / 1000 < TIMING);
            }
            int valid = 1;
            for (int i = 0; i < 14; i++)
            {
                if (buffer[i * 2 + 1] + buffer[i * 2] != 1)
                {
                    valid = 0;
                }
            }

            if (valid)
            {
                for (int i = 0; i < 28; i++)
                {
                    printf(" %d", buffer[i]);
                }
                printf("\n");
                fflush(stdout);
            }
        }
    }
}
