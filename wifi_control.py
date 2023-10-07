import subprocess

def is_enabled():
    return subprocess.run("nmcli radio wifi", shell=True, stdout=subprocess.PIPE).stdout.decode('ascii') == "enabled\n"

def enable():
    subprocess.run("nmcli radio wifi on", shell=True)

def disable():
    subprocess.run("nmcli radio wifi off", shell=True)

def get_current_connection():
    connections = subprocess.run("nmcli -f NAME,DEVICE connection | grep wlo1", shell=True, stdout=subprocess.PIPE).stdout.split()
    if len(connections) > 0:
        return connections[0]
    else:
        return None

def get_available_connections():
    # security WPA1 WPA2 --
    connections = subprocess.run("nmcli -c no -e no -f SSID,SIGNAL,SECURITY device wifi list --rescan yes", shell=True, stdout=subprocess.PIPE).stdout.decode('ascii').split("\n")[1:]
    return connections

    if len(connections) > 0:
        return subprocess.run("nmcli connection | grep wlo1", shell=True, stdout=subprocess.PIPE).stdout.split()[0]
    else:
        return None

'''
nmcli connection
nmcli device wifi list 
nmcli device wifi list --rescan yes
nmcli device wifi list --rescan yes
nmcli c up iPad (Влад)
nmcli c up 'iPad (Влад)'
nmcli radio wifi off
nmcli radio wifi on
nmcli c delete JioFi4_12E9FE
'''