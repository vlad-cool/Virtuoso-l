#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 200000

int btns[] = {26, 37, -1};
int toggle[] = {15, -1}

void flush()
{
    while (getchar() != '\n');
    printf("Not a number\n");
}

void setup()
{
    wiringPiSetupPhys();
    for (int *i = btns; *i != -1; i++)
    {
        pinMode(*i, 1);
        digitalWrite(*i, 1);
    }
}

void send(int pin)
{
    for (int *i = btns; *i != -1; i++)
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
    for (int *i = toggle; *i != -1; i++)
    {
        if (*i == pin / 10)
        {
            digitalWrite(pin / 10, pin % 10);
            return;
        }
    }
    printf("Unregistered pin! See source; ");
}

int main()
{
    setup();
    int pin = 0, scanf_res = 0;

    while (pin != -1)
    {
        while ((scanf_res = scanf("%d", &pin)) == 0)
            flush();
        if (scanf_res == EOF || btn == -1)
            return 0;
        
        if (pin >= 0)
        {
            send(pin);
        }
            
        printf("%d\n", pin);
    }
    printf("Exiting!\n");
}