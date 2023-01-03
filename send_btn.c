#include <wiringPi.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define TIMING 200000

int *btns = {27, 37, -1}

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
            usleep(TIMING)
            digitalWrite(btn, 0);
            usleep(TIMING)
            digitalWrite(btn, 1);
            return;
        }
    }
    printf("Unregistered button! See source\n");
}


int main()
{
    int btn = 0;

    while (btn != -1)
    {
        FILE *f = fopen("send_btn_fifo", "r");
        if (btn > 0)
        {
            send(btn);
        }
        printf("%d\n", btn);
        fclose(f);
    }
    printf("Exiting!\n");
}