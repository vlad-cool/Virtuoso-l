if [ -z "$SSH_TTY" ]
then
    cd V24m
    startx ./app.py
else
    echo "Hello, remote SSH user!"
fi
