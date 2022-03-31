#!/bin/bash

sleep 30 && docker cp Bob:/home/Bob/clef_pub_Bob clef_pub_Bob && docker cp clef_pub_Bob John:/home/John/clef_pub_Bob &

docker network create -d bridge --subnet 172.10.0.0/16 John-training-lab

docker run --net John-training-lab --ip 172.10.0.2 --name Bob --hostname Bob bob

docker run --net John-training-lab --ip 172.10.0.3 -it --name John --hostname John john

#Ne marche pas, fait le cp une fois que le container John a était quitté: 
#docker cp Bob:/home/Bob/clef_pub_Bob clef_pub_Bob && docker cp clef_pub_Bob John:/home/John/clef_pub_Bob && echo cp OK || echo cp pas OK


