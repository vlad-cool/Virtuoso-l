//Compile with `gcc -o output send_data.c -lwiringPi`

#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 889

int main(int argc, char *argv[])
{
    wiringPiSetupPhys();
    pinMode(26, 1);
    digitalWrite(26, 1);
    sleep(0.1);

    printf("%d\n", argc);
    int toggle_bit = 1, data[14], to_transmit;
    unsigned long time;
    struct timespec t;
    data[0] = 1;
    data[1] = 1;
    for (int i = 1; i < argc; i++)
    {
        data[2] = toggle_bit;
        to_transmit = atoi(argv[i]);

        for (int j = 0; j < 11; j++)
        {
            data[13 - j] = to_transmit % 2;
            to_transmit /= 2;
        }
        toggle_bit = !toggle_bit;

        for (int j = 0; j < 14; j++)
        {
            printf("%d ", data[j]);
        }

        int j = 0;
        printf("\n!\n");
        clock_gettime(CLOCK_BOOTTIME, &t);
        while (j < 14)
        {
            time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
            digitalWrite(26, data[j]);

            while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
            {
                clock_gettime(CLOCK_BOOTTIME, &t);
            }

            time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;
            digitalWrite(26, !data[j]);

            while (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time < TIMING)
            {
                clock_gettime(CLOCK_BOOTTIME, &t);
            }
            j++;
        }
        digitalWrite(26, 1);
        sleep(0.05);
    }
}
