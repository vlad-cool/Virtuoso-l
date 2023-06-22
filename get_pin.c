#include <wiringPi.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

int pins[] = {3, 7, 18, 27, 32, 36, -1};

void flush()
{
    while (getchar() != '\n');
}

void setup()
{
    wiringPiSetupPhys();
    for (int *i = pins; *i != -1; i++)
    {
        pinMode(*i, 0);
    }
}

int main()
{
    setup();
    char *s = calloc(sizeof(char), 256);
    
    while (1)
    {
        if (scanf("%128s", s) < 1)
        {
            break;
        }
        if (strcmp(s, "get") == 0)
        {
            printf("{");
            for (int *i = pins; *i != -1; i++)
            {
                printf("%d: %d, ", *i, digitalRead(*i));
            }
            printf("}\n");
            fflush(stdout);
        }
        if (strcmp(s, "exit") == 0)
        {
            break;
        }
    }
}