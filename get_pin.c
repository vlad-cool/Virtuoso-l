#include <wiringPi.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <sys/poll.h>

int pins[] = {3, 7, 27, 32, 36};

int pin_18_state_buf[8] = {0};

void flush()
{
    while (getchar() != '\n');
}

void setup()
{
    wiringPiSetupPhys();
    for (int i = 0; i < sizeof(pins) / sizeof(int); i++)
    {
        pinMode(pins[i], 0);
    }
    pinMode(18, 0);
}

int main()
{
    setup();
    char *s = calloc(sizeof(char), 256);
    struct timespec t;
    long long time = 0;

    while (1)
    {
        struct pollfd fds;
        int ret;
        fds.fd = 0;
        fds.events = POLLIN;
        ret = poll(&fds, 1, 0);
        if(ret == 1)
        {
            if (scanf("%128s", s) < 1)
            {
                break;
            }
            if (strcmp(s, "get") == 0)
            {
                printf("{");
                for (int i = 0; i < sizeof(pins) / sizeof(int); i++)
                {
                    printf("%d: %d, ", pins[i], digitalRead(pins[i]));
                }

                int pin_18_state = 0;
                for (int i = 0; i < sizeof(pin_18_state_buf) / sizeof(int); i++)
                {
                    pin_18_state += pin_18_state_buf[i];
                }
                printf("%d: %d, ", 18, pin_18_state == 0 ? 0 : 1);

                printf("}\n");
                fflush(stdout);
            }
            if (strcmp(s, "exit") == 0)
            {
                break;
            }
        }
        clock_gettime(CLOCK_BOOTTIME, &t);

        if (t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000 - time >5000)
        {
            time = t.tv_sec * 1000 * 1000 + t.tv_nsec / 1000;

            int i;

            for (i = 0; i + 1 < sizeof(pin_18_state_buf) / sizeof(int); i++)
            {
                pin_18_state_buf[i] = pin_18_state_buf[i + 1];
            }
            pin_18_state_buf[i] = digitalRead(18);
        }
    }
}