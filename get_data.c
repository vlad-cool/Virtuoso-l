//Compile with `gcc -o input get_data.c -lwiringPi`

#include <wiringSerial.h>
#include <wiringPi.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

void setup() {

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

int main() {
    int byte;
    int err = 1;
    int k = 0;
    char data[9];
    int desc = serialOpen("/dev/ttyS2", 38400);
    setup();

    while (1)
    {
        for (int i = 0; i < 9; i++)
        {
            data[i] = 0;
        }

        printf("%d\n", serialDataAvail(desc));
        int available = serialDataAvail(desc);
        for (int i = 0; i < available; i++)
        {
            err = 0;
            byte = serialGetchar(desc);
            data[byte / 32 + 1] = byte;
        }

        /*
        if (err)
        {
            FILE * pipe = fopen("./gpio_in", "rb");

            for (int i = 1; i < 9; i++)
            {
                fread(data + i, 1, 1, pipe);
            }

            fclose(pipe);
        }
        */

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

        FILE * pipe = fopen("./gpio_in", "wb");

        for (int i = 0; i < 9; i++)
        {
            fwrite(&data[i], 1, 1, pipe);
            printf("data %d: %d\n", i, data[i]);
        }

        fclose(pipe);
        err = 1;
        usleep(500 * 1000);
    }

    serialClose(desc);
}
