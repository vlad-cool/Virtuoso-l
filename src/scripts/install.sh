#!/bin/sh
rm -rf V24m/app
mv V24m/update.zip V24m_update
unzip V24m_update.zip
mv app V24m/
rm V24m_update.zip
