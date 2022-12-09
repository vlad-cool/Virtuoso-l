#!/usr/bin/python3
import select
import sys
import cv2
from platform        import machine
from multiprocessing import Process

from time import sleep

running = True
pause   = False
vid     = None

# show
# hide
# stop

# err1 - camera opening error

def run_in_thread(func):
    def inner1(*args, **kwargs):
        thread = Process(target=func, args=args, kwargs=kwargs)
        thread.start()
    return inner1

def input_handler():
    global running
    global pause
    s = input()
    if s == "show":
        pause   = False
        #init()
    if s == "hide":
        pause   = True
        #cv2.destroyAllWindows()
    if s == "stop":
        pause   = False
        running = False
    if s == "move":
        cv2.moveWindow("cam_output", 100, 0)

def avail():
    rlist, _, _ = select.select(
            [sys.stdin], [], [], 0
        )
    return bool(rlist)

def init():
    global vid
    if machine() == "armv7l":
        vid = cv2.VideoCapture(0)
    else:
        vid = cv2.VideoCapture(0)
    
    if not vid.isOpened(): 
        print("err1")

#@run_in_thread
def run():
    global running
    global pause
    global vid
    
    cv2.moveWindow("cam_output", 200, 0)

    while running:
        ret, frame = vid.read()
        if ret == True:
            frame = cv2.rotate(frame, cv2.ROTATE_90_CLOCKWISE)
            cv2.imshow("cam_output", frame)
            if avail():
                input_handler()
                while pause:
                    input_handler()
            if cv2.waitKey(25) & 0xFF == ord('q'):
                break

init()
run()

vid.release()
cv2.destroyAllWindows()
