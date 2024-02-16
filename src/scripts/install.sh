#!/bin/sh
cp V24m/V24m_update.zip V24m_update.zip
unzip V24m_update.zip
if [ $? -eq 0 ]; then
    rm -rf V24m/app
    mv app V24m/
    rm V24m_update.zip
    rm V24m/V24m_update.zip
else
    echo -e "\033[1;31;43mUnzipping failed, rebooting\033[0m"
    sleep 10
fi
