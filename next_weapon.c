//Compile with `gcc -o weapon next_weapon.c -lwiringPi`

#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING (300 * 1000)

void setup()
{
    wiringPiSetupPhys();
    pinMode(37, 1);
    digitalWrite(37, 1);
}

void send(int times)
{
    unsigned long time;
    struct timespec t;

    clock_gettime(CLOCK_BOOTTIME, &t);

    for (int i = 0; i < times; i++)
    {
        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
        digitalWrite(37, 0);

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }

        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
        digitalWrite(37, 1);

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }
    }
}

int main(int argc, char *argv[])
{
    setup();
    send(atoi(argv[1]));
}
