#include "gpio_lib.hpp"
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 889
#define rc5_pin 26

int ir_toggle_bit = 1;

void flush()
{
    while (getchar() != '\n');
}

void setup()
{
    gpioSetupPhys();
    pinMode(rc5_pin, 1);
    digitalWrite(rc5_pin, 1);
}

void send(int address, int command)
{
    unsigned long time;
    struct timespec t;
    int to_transmit = (address * (2 << 5) + command);
    to_transmit += (1 << 13) + (1 << 12);
    to_transmit += ir_toggle_bit * (1 << 11);
    int data[14];

    for (int i = 0; i < 14; i++)
    {
        data[13 - i] = to_transmit % 2;
        to_transmit /= 2;
    }

    clock_gettime(CLOCK_BOOTTIME, &t);

    for (int i = 0; i < 14; i++)
    {
        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
        digitalWrite(rc5_pin, 0 + data[i]);

        usleep(TIMING * 9 / 10);

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }

        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
        digitalWrite(rc5_pin, 1 - data[i]);

        usleep(TIMING * 9 / 10);

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }
    }

    digitalWrite(rc5_pin, 1);
    usleep(TIMING * 100);
}

int main()
{
    setup();
    int address, command;
    char s[128];
    
    while (1)
    {
        if (scanf("%127s", s) < 1)
        {
            break;
        }
        if (strcmp(s, "transmit") == 0)
        {
            if (scanf("%d %d", &address, &command) < 2)
                flush();
            else
                send(address, command);
            continue;
        }
        if (strcmp(s, "ping") == 0) // used to wait until transmitted for address evaluation
        {
            printf("pong\n");
            fflush(stdout);
        }
        if (strcmp(s, "exit") == 0)
        {
            break;
        }
    }
}