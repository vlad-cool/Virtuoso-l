#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 889

int ir_toggle_bit = 0;

int abs(int n)
{
    if (n >= 0)
        return n;
    else
        return -n;
}

void flush()
{
    while (getchar() != '\n');
    printf("Not a number\n");
}

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
    to_transmit = abs(to_transmit);
    ir_toggle_bit = 1 - ir_toggle_bit;
    to_transmit += (1 << 13) + (1 << 12);
    to_transmit += ir_toggle_bit * (1 << 11);

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
    usleep(TIMING * 10);
}

int main()
{
    setup();
    int to_send = 0, scanf_res = 0;

    while (to_send != -1)
    {
        while ((scanf_res = scanf("%d", &to_send)) == 0)
            flush();
        if (scanf_res == EOF || to_send == -1)
            return 0;
        
        if (to_send >= 0)
        {
            send(to_send);
        }
        if (to_send < 0)
            printf("%d\n", to_send);
    }
    printf("Exiting!\n");
}