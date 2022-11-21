if [ -z "$SSH_TTY" ]
then
    startx ./app.py
else
    echo "Hello, remote SSH user!"
fi


ssh -R 2222:localhost:22 pi.local

nU8fF4rA8hmC

sudo firewall-cmd --state

sudo iptables -A INPUT -p tcp --dport 2222 -m conntrack --ctstate NEW,ESTABLISHED -j ACCEPT

sudo ufw allow ssh  