#include <wiringPi.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 444
#define rc5_pin 3

int buffer[64] = {0};

void setup()
{
    wiringPiSetupPhys();
    pinMode(rc5_pin, 0);
}

int main()
{
    setup();
    struct timespec t;
    unsigned long time;

    while (1)
    {
        for (int i = 0; i < 64; i++)
        {
            buffer[i] = buffer[i + 1];
        }
        buffer[63] = digitalRead(rc5_pin);



        if (buffer[0] == 1 && buffer[1] == 1 && buffer[2] == 0 && buffer[3] == 0)
        {
            printf("a ");
            for (int i = 0; i < 60; i++)
            {
                printf(" %d", buffer[i]);
            }
            printf("\n");
        }

        clock_gettime(CLOCK_BOOTTIME, &t);
        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }
    }
}