#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 200000

int btns[] = {26, 37, -1};

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

void send(int btn)
{
    for (int *i = btns; *i != -1; i++)
    {
        if (*i == btn)
        {
            usleep(TIMING);
            digitalWrite(btn, 0);
            usleep(TIMING);
            digitalWrite(btn, 1);
            return;
        }
    }
    printf("Unregistered button! See source; ");
}

int main()
{
    setup();
    int btn = 0, scanf_res = 0;

    while (btn != -1)
    {
        while ((scanf_res = scanf("%d", &btn)) == 0)
            flush();
        if (scanf_res == EOF || btn == -1)
            return 0;
        
        if (btn >= 0)
        {
            send(btn);
        }
            
        printf("%d\n", btn);
    }
    printf("Exiting!\n");
}