//Compile with `gcc -o test wiringpi.c -lwiringPi`

#include <wiringPi.h>
#include <stdio.h>

int main (void)
{
    wiringPiSetupPhys();
    
    // AtMega programming pins
    pinMode(8, 1);
    pinMode(10, 1);
    pinMode(12, 1);
    pinMode(16, 1);
    pinMode(19, 1);
    pinMode(21, 1);
    pinMode(23, 1);
    pinMode(24, 1);

    
    digitalWrite(8, 1);
    digitalWrite(10, 1);
    digitalWrite(12, 1);
    digitalWrite(16, 1);
    digitalWrite(19, 1);
    digitalWrite(21, 1);
    digitalWrite(23, 1);
    digitalWrite(24, 1);

    // Uart (11) (WIP)
    pinMode(11, 0);

    // Recording
    pinMode(18, 0);

    // IR remote emulation
    pinMode(26, 0);
  //  digitalWrite(26, 1);
    
    // Weapon select
    pinMode(37, 0);

    // Weapon type
    pinMode(32, 0);
    pinMode(36, 0);
    
    // Weapon connection type
    pinMode(11, 0);

    printf("Written!\n");
    while (1)
    {
        switch (digitalRead(32) * 2 + digitalRead(36))
        {
        case 1:
            printf("sa ");
            break;

        case 2:
            printf("sp ");
            break;

        case 3:
            printf("ra ");
            break;
        
        default:
            printf("er ");
            break;
        }

        if (digitalRead(7))
        {
            printf("wired\n");
        }
        else
        {
            printf("wireless\n");
        }

        if (digitalRead(18))
        {
            printf("r\n");
        }
        else
        {
            printf("s\n");
        }

        if (digitalRead(37))
        {
            printf("37 - 1\n");
            //pinMode(26, 0);
        }
        else
        {
            printf("37 - 0\n");
            //pinMode(26, 1);
            //digitalWrite(26, 1);
        }

        int qqq = 0, p = 0, r;
        while (!digitalRead(37))
        {
            r = digitalRead(26);
            printf("%i", r);
            if (p != r)
            {
                qqq++;
            }
            p = r;
        }
        printf("asasd fd %i\n", qqq);
        getchar();
    }
    return 0;
}