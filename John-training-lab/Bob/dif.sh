#!/bin/bash
echo $1
ssh-keygen -b 1024 -P $2 -f /root/.ssh/id_rsa && cat /root/.ssh/id_rsa.pub > /home/Bob/clef_pub_Bob && chmod 600 /home/Bob/clef_pub_Bob && sleep 60 #&& scp /home/Bob/clef_pub_Bob John@172.10.0.2:/home/John/clef_pub_Bob