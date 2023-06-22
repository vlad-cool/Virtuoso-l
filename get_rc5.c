#include <wiringPi.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 889
#define rc5_pin 3

int buffer[30] = {0};

void flush()
{
    while (getchar() != '\n');
}

void setup()
{
    wiringPiSetupPhys();
    pinMode(rc5_pin, 0);
}

int main()
{
    setup();
    
    while (1)
    {
        for (int i = 0; i < 27; i++)
        {
            buffer[i] = buffer[i + 1];
        }
        buffer[27] = digitalRead(rc5_pin);

        //printf("%d ", buffer[27]);

        //if (buffer[0] == 1 && buffer[1] == 0 && buffer[2] == 1 && buffer[3] == 0 && buffer[28] == 1 && buffer[29] == 1)
        if (buffer[0] == 1 && buffer[1] == 0 && buffer[2] == 1 && buffer[3] == 0)
        {
            printf("+");
            for (int i = 0; i < 30; i++)
            {
                printf(" %d", buffer[i]);
            }
            printf("\n");
        }
        //if (buffer[0] == 0 && buffer[1] == 1 && buffer[2] == 0 && buffer[3] == 1 && buffer[28] == 1 && buffer[29] == 1)
        if (buffer[0] == 0 && buffer[1] == 1 && buffer[2] == 0 && buffer[3] == 1)
        {
            printf("-");
            for (int i = 0; i < 30; i++)
            {
                printf(" %d", buffer[i]);
            }
            printf("\n");
        }

        usleep(TIMING);
    }
}