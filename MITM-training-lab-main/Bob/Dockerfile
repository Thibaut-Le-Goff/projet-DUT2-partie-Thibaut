FROM debian:stretch

# mise à jour du système
RUN apt-get update -y \
    && apt-get upgrade -y \
    # installation des paquets nécessaires à la 
    # réalisation de l'épreuve
    && apt-get install -y apt-utils \
    && apt-get install -y iputils-ping \
    && apt-get install -y iptables \
    # Création de l'utilisateur Alice
    && useradd -ms /bin/bash Bob

# ajout du fichier bash qui sert à ping Alice pour transmetre le flag,
# et à ping Eve, le container que le participant contrôlera, afin qu'Alice
#sois enregistré dans sa table arp afin de retrouver son addresse
#ip via un ipneigh

ADD ping.sh /ping.sh
RUN chmod 755 /ping.sh \
    # Change d'utilisateur et passage dans sont répertoire 
    # afin que le container soit présentable,
    # si le participant rentre dans le container
    && echo "cd /home/Alice && su Bob" >> /home/Bob/.bashrc

# suite d'espaces, les 20, afin d'éviter que le participant évite
# de prendre les données transmissent comme étant
# le flag qui est : 464c41473a205028297c4f20
# le Dockerfile afin d'éviter que le participant le trouve 
# en lissant le fichier ping.sh 
CMD ["./ping.sh", "202020202020202020202020", "464c41473a205028297c4f20"]
