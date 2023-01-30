#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

int prog_pins[] = {8, 10, 12, 16, 19, 21, 23, 24, -1};
int pins[] = {7, 18, 32, 36, -1};

void flush()
{
    while (getchar() != '\n');
}

void setup()
{
    wiringPiSetupPhys();
    for (int *i = prog_pins; *i != -1; i++)
    {
        pinMode(*i, 1);
        digitalWrite(*i, 1);
    }
    for (int *i = pins; *i != -1; i++)
    {
        pinMode(*i, 0);
    }
}

int main()
{
    setup();
    int request = 0, scanf_res = 0;

    while (request != -1)
    {
        while ((scanf_res = scanf("%d", &request)) == 0)
            flush();
        if (scanf_res == EOF || request == -1)
            return 0;
        printf("{");
        for (int *i = pins; *i != -1; i++)
        {
            printf("%d: %d, ", *i, digitalRead(*i));
        }
        printf("}\n");
        fflush(stdout);
    }
}