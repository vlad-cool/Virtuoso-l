//Compile with `gcc -o input get_data.c -lwiringPi`

#include <wiringSerial.h>
#include <wiringPi.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

void setup()
{
    wiringPiSetupPhys();

    pinMode(7, 0);
    pinMode(18, 0);
    pinMode(32, 0);
    pinMode(36, 0);

    pinMode(8, 1);
    pinMode(10, 1);
    pinMode(12, 1);
    pinMode(16, 1);
    pinMode(19, 1);
    pinMode(21, 1);
    pinMode(23, 1);
    pinMode(24, 1);

    pinMode(26, 1);

    digitalWrite(8, 1);
    digitalWrite(10, 1);
    digitalWrite(12, 1);
    digitalWrite(16, 1);
    digitalWrite(19, 1);
    digitalWrite(21, 1);
    digitalWrite(23, 1);
    digitalWrite(24, 1);

    digitalWrite(26, 1);
}

void get_data(int descriptor, const char *file_name)
{
    int err = 0;
    char data[] = {0, 0, 0, 0, 0, 0, 0, 0, 0};
    int byte;
    printf("data avail          %d\n", serialDataAvail(descriptor));
    while(serialDataAvail(descriptor) != 0)
    {
        byte = serialGetchar(descriptor);
        printf("                  byte: %d\n");
        data[byte / 32] = byte;
    }

    if (data[8] == 0)
    {
        FILE * pipe = fopen(file_name, "rb");
        err = 1;

        for (int i = 0; i < 9; i++)
        {
            fread(&data[i], 1, 1, pipe);
        }
        fclose(pipe);
    }

    data[0] += 1;
    data[0] <<= 1;
    data[0] += 1;
    data[0] <<= 1;
    data[0] += 1;
    data[0] <<= 1;
    data[0] += err;
    data[0] <<= 1;
    data[0] += digitalRead(7);
    data[0] <<= 1;
    data[0] += digitalRead(18);
    data[0] <<= 1;
    data[0] += digitalRead(32);
    data[0] <<= 1;
    data[0] += digitalRead(36);

    FILE * pipe = fopen(file_name, "wb");

    for (int i = 0; i < 9; i++)
    {
        fwrite(&data[i], 1, 1, pipe);
       // printf("data %d: %d\n", i, data[i]);
    }

    fclose(pipe);
}

int main(void)
{
    setup();
    int descriptor = serialOpen("/dev/ttyS2", 38400);

    while (1)
    {
        get_data(descriptor, "./gpio_in");
        sleep(1);
    }

    serialClose(descriptor);
}
