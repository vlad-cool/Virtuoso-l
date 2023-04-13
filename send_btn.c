#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 200000

int active = {8, 10, 12, 16, 19, 21, 23, 24, -1};
int out[] = {15, 26, 37, -1};
int ini[] = { 0,  1,  1, -1}

void flush()
{
    while (getchar() != '\n');
    printf("Not a number\n");
}

void setup()
{
    wiringPiSetupPhys();
    for (int i = 0; out[i] != -1; i++)
    {
        pinMode(out[i], 1);
        digitalWrite(out[i], ini[i]);
    }
}

void btn(int pin)
{
    for (int *i = out; *i != -1; i++)
    {
        if (*i == pin)
        {
            usleep(TIMING);
            digitalWrite(pin, 0);
            usleep(TIMING);
            digitalWrite(pin, 1);
            return;
        }
    }
    printf("Unknown pin!\n");
}

void set(int pin, int val)
{
    for (int *i = out; *i != -1; i++)
    {
        if (*i == pin)
        {
            digitalWrite(pin, val);
            return;
        }
    }
    printf("Unknown pin!\n");
}

int main()
{
    setup();
    int pin = 0, scanf_res = 0, val = 0;
    char com;

    while (pin != -1)
    {
        while ((scanf_res = scanf("%c %d", &pin)) < 2)
            flush();
        if (scanf_res == EOF || c == 'e')
            return 0;
        
        if (c == 'b')
        {
            btn(pin);
        }
        else if (c == 's')
        {
            scanf_res = scanf("%d", &val);
            if (scanf_res == -1)
                return 0;
            else if (scanf_res == 0)
                continue;
            else
                set(pin, val);
        }

        printf("%d\n", pin);
    }
    printf("Exiting!\n");
}