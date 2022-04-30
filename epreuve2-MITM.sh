#!/bin/bash

docker network create --subnet 172.50.0.0/16 MITM

	docker pull ctfiutbeziers/m-alice && 
	docker run --net MITM --ip 172.50.0.2 --cap-add=NET_ADMIN --name alice --hostname MITM ctfiutbeziers/m-alice &

docker pull ctfiutbeziers/m-bob && 
	docker run --net MITM --ip 172.50.0.3 --cap-add=NET_ADMIN --name bob --hostname MITM ctfiutbeziers/m-bob &

docker pull ctfiutbeziers/m-eve && 
	docker run --net MITM --ip 172.50.0.4 --cap-add=NET_ADMIN -it --name eve --hostname MITM ctfiutbeziers/m-eve
