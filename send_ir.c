#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 889

void setup()
{
    wiringPiSetupPhys();
    pinMode(26, 1);
    digitalWrite(26, 1);
}

void send(int to_transmit)
{
    int data[14];
    unsigned long time;
    struct timespec t;

    for (int i = 0; i < 14; i++)
    {
        data[13 - i] = to_transmit % 2;
        to_transmit /= 2;
    }

    clock_gettime(CLOCK_BOOTTIME, &t);

    for (int i = 0; i < 14; i++)
    {
        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
        digitalWrite(26, 0 + data[i]);

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }

        time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
        digitalWrite(26, 1 - data[i]);

        while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
        {
            clock_gettime(CLOCK_BOOTTIME, &t);
        }
    }

    digitalWrite(26, 1);
    usleep(TIMING * 1000 * 10);
}


int main()
{
    int to_send = 0;

    while (to_send != -1)
    {
        FILE *f = fopen("send_ir_fifo", "r");
        if (to_send > 0)
        {
            send(to_send);
        }
        printf("%d\n", to_send);
        fclose(f);
    }
    printf("Exiting!\n");
}