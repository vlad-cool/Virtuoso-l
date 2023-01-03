#!/bin/sh
gcc send_ir.c -o send_ir
chmod 4711 send_ir
gcc send_ir.c -o send_btn
chmod 4711 send_btn