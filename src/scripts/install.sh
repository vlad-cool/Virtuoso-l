#!/bin/sh
cd V24m
rm -rf app_old
mv app app_old
unzip V24m_update.zip
if [ $? -eq 0 ]; then
    rm -rf app_old
    rm V24m_update.zip
    echo "\033[1;31;43mUpdate successful, continuing\033[0m"
    sleep 10
else
    echo "\033[1;31;43mUpdate failed, reverting\033[0m"
    rm -rf app
    mv app_old app
    sleep 10
fi
