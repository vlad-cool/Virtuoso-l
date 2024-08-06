#!/bin/sh
cd Virtuoso
rm -rf app_old
mv app app_old
unzip Virtuoso_update.zip
if [ $? -eq 0 ]; then
    rm -rf app_old
    rm Virtuoso_update.zip
    echo "\033[1;31;43mUpdate successful, continuing\033[0m"
else
    echo "\033[1;31;43mUpdate failed, reverting\033[0m"
    rm -rf app
    mv app_old app
fi
