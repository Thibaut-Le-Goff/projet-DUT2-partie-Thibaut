#!/bin/bash

#Bloque les echos reply afin que le participant soit obligé
#de faire deux arpspoof, le flag que Bob envoie à Alice est
#visible dans l'echo reply qu'Alice envoie à Bob si le
#participant ne fait qu'un arpspoof en se faisant passer pour
#Bob.

iptables -A OUTPUT -p icmp --icmp-type echo-reply -j DROP && su Bob

for (( ; ; ))
do
        ping -c 1 -p $1 172.17.0.4 > /dev/null & ping -c 1 -p $2 172.17.0.2 > /dev/null
done
