import subprocess
from time import sleep

def get_address():
    spacing_time = .3
    data = []
    for i in range(9):
        data.append([0] * 8)
    with open("./gpio_in", "rb") as gpio_in:
        for j in range(9):
            b = gpio_in.read(1)
            a = int.from_bytes(b, "big")
            if a == 0 and j == 0:
                return
            for i in range(8):
                data[j][7 - i] = a % 2
                a //= 2
            if j == 0 and data[0][3] == 1:
                break
    val = data[5][7]
    timer = data[3][3]
    if timer:
        command = 13 #timer start stop
    elif val:
        command = 3  #left -
    else:
        command = 2  #left +


    for k in range(32):
        subprocess.run(["sudo", "./ir_emu", str(k * (2**6) + command)])
        sleep(spacing_time + timer)
        for j in range(9):
            data[j] = [0] * 8
        with open("./gpio_in", "rb") as gpio_in:
            for j in range(9):
                b = gpio_in.read(1)
                a = int.from_bytes(b, "big")
                if a == 0 and j == 0:
                    return
                for i in range(8):
                    data[j][7 - i] = a % 2
                    a //= 2
                if j == 0 and data[0][3] == 1:
                    break
        if (val != data[5][7] or timer != data[3][3]):
            if timer:
                subprocess.run(["sudo", "./ir_emu", str(k * (2**6) + command)])
            else:
                subprocess.run(["sudo", "./ir_emu", str(k * (2**6) + 5 - command)])

            return k
        print(k, data)
    return -1

if __name__ == "__main__":
    print(get_address())
