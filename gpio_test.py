#!/usr/bin/env python3 
import wiringpi2
from time import sleep

wiringpi2.wiringPiSetupGpio()

wiringpi2.pinMode(8,1)         #AtMega programming pins
wiringpi2.pinMode(10,1)        #AtMega programming pins
wiringpi2.pinMode(12,1)        #AtMega programming pins
wiringpi2.pinMode(16,1)        #AtMega programming pins
wiringpi2.pinMode(1,1)         #AtMega programming pins
wiringpi2.pinMode(21,1)        #AtMega programming pins
wiringpi2.pinMode(23,1)        #AtMega programming pins
wiringpi2.pinMode(24,1)        #AtMega programming pins

wiringpi2.digitalWrite(8, 1)   #AtMega programming pins
wiringpi2.digitalWrite(10, 1)  #AtMega programming pins
wiringpi2.digitalWrite(12, 1)  #AtMega programming pins
wiringpi2.digitalWrite(16, 1)  #AtMega programming pins
wiringpi2.digitalWrite(1, 1)   #AtMega programming pins
wiringpi2.digitalWrite(21, 1)  #AtMega programming pins
wiringpi2.digitalWrite(23, 1)  #AtMega programming pins
wiringpi2.digitalWrite(24, 1)  #AtMega programming pins

#####UART (wip)

wiringpi2.pinMode(18, 0)       #Recording

wiringpi2.pinMode(26, 1)       #IR controller emulation
wiringpi2.digitalWrite(26, 1)  #IR controller emulation

wiringpi2.pinMode(37, 0)       #Weapon select

wiringpi2.pinMode(32, 0)       #Weapon type 0
wiringpi2.pinMode(36, 0)       #Weapon type 1
wiringpi2.pinMode(7, 0)        #Weapon connection type


wiringpi2.pinMode()

loader = "*-----"

while True:
    print(loader)
    loader = loader[-1] + loader[:-1:]
    #weapon type
    print(["рапира", "шпага", "сабля", "ошибка (не используется)"][::-1][wiringpi2.digitalRead(32) * 2 + wiringpi2.digitalRead(36)])
    #weapon connection type
    print(["беспроводной", "проводной"][wiringpi2.digitalRead(7)])
    #recording function
    print("запись: ", ["low", "high"][wiringpi2.digitalRead(18)])
    print("################")

    sleep(.5)