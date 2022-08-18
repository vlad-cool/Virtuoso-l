#include <wiringSerial.h>
#include <wiringPi.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

int main(void) {
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

    
    digitalWrite(8, 1);
    digitalWrite(10, 1);
    digitalWrite(12, 1);
    digitalWrite(16, 1);
    digitalWrite(19, 1);
    digitalWrite(21, 1);
    digitalWrite(23, 1);
    digitalWrite(24, 1);

    FILE * pipe = fopen("./gpio_in", "wb");
    fclose(pipe);

    int byte;
    int err = 0;
    int k = 0;
    char data[9];
    int desc = serialOpen("/dev/ttyS2", 38400);
    printf("OBABO\n");


    while (1)
    {
        for (int i = 0; i < 9; i++)
        {
            data[i] = 0;
        }
        //err = serialDataAvail(desc) == 0;
        while(err == 0 && data[7] < 224)
        {
            for (int i = 0; i < 7; i++)
            {
                data[i] = data[i + 1];
            }
            byte = serialGetchar(desc);
            if (byte == -1)
            {
                //serialClose(desc); //try without this line
                err = 1;
                //int desc = serialOpen("/dev/ttyS2", 38400); //try without this line
            }
            else
            {
                data[7] = byte;
            }
        }
        
        data[8] += 1;
        data[8] <<= 1;
        data[8] += 1;
        data[8] <<= 1;
        data[8] += 1;
        data[8] <<= 1;
        data[8] += err;
        data[8] <<= 1;
        data[8] += digitalRead(7);
        data[8] <<= 1;
        data[8] += digitalRead(18);
        data[8] <<= 1;
        data[8] += digitalRead(32);
        data[8] <<= 1;
        data[8] += digitalRead(36);

        FILE * pipe = fopen("./gpio_in", "wb");
        fwrite(&data[8], 1, 1, pipe);
        printf("data %d: %d\n", 8, data[8]);
        for (int i = 0; i < 8; i++)
        {
            fwrite(&data[i], 1, 1, pipe);
            printf("data %d: %d\n", i, data[i]);
        }
        printf("ABOBA\n");
        fclose(pipe);
        err = 0;
        sleep(.3);
    }

    printf("FINISH\n");
    serialClose(desc);
}