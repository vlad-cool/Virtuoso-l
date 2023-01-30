#!venv/bin/python3
import serial

data_rx = serial.Serial("/dev/ttyS2", 38400)

def byte_to_arr(byte):
    a = [0] * 8
    for i in range(8):
            a[i] = byte % 2
            byte //= 2
    return a[::-1]

def get_data():
    global data_rx
    data = [[0] * 8] * 8
    while data_rx.inWaiting() // 8 > 0:
        for i in range(8):
            byte = int.from_bytes(data_rx.read(), "big")
            data[byte // 2 ** 5] = byte_to_arr(byte)

        print("data_got!")
        print(str(data).replace("], ", "]]\n["))


        period = 0
        timer_m = 0
        timer_d = 0
        timer_s = 0
        
        for i in range(4):
            period *= 2
            period += data[6][4 + i]

        for i in data[1][6:]:
            timer_m *= 2
            timer_m += i

        for i in data[2][4:]:
            timer_d *= 2
            timer_d += i

        for i in data[3][4:]:
            timer_s *= 2
            timer_s += i
        
        print(f"period: {period}")
        print(f"timer:  {timer_m}:{timer_d}{timer_s}")

while True:
    get_data()