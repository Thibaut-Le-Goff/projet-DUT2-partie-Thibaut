#!/bin/bash

#Bloque les echos reply afin que le participant soit obligé
#de faire deux arpspoof, le flag qu'Alice envoie à Bob est
#visible dans l'echo reply que Bob envoie à Alice si le
#participant ne fait qu'un arpspoof en se faisant passer pour
#Alice.

iptables -A OUTPUT -p icmp --icmp-type echo-reply -j DROP && su Alice

for (( ; ; ))
do
	ping -c 1 -p $1 172.17.0.4 > /dev/null & ping -c 1 -p $2 172.17.0.3 > /dev/null
done
