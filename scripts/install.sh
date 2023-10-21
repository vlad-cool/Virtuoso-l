#!/bin/bash
VERSION=0.0.1
cd V24m
rm !(main.kv)
cd
tar xvf 0.0.1.tar.gz -C V24m/
mv V24m/bin/* V24m/
rm -r V24m/bin
sudo chown root get_pin send_pin get_rc5 send_rc5
sudo chmod 4755 get_pin send_pin get_rc5 send_rc5