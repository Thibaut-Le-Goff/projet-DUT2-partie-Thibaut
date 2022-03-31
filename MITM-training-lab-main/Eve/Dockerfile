FROM debian:stretch

# mise à jour du système
RUN apt-get update -y \
    && apt-get upgrade -y \
    && apt-get install -y apt-utils \
    # installation des paquets nécessaires à la réalisation de l'épreuve
    && apt-get install -y iproute2 \
    && apt-get install -y net-tools \
    && apt-get install -y dsniff \
    && apt-get install -y dhcpdump \
    && apt-get install -y iputils-ping \
    && apt-get install -y sudo \
    && apt-get install -y hping3 \
    && apt-get install -y tshark \
    # Création de l'utilisateur Eve
    && useradd -ms /bin/bash Eve \
    # Donne le mot de passe "Eve" à Eve
    && echo Eve:Eve | chpasswd \
    # ajout d'Eve dans le groupe sudo afin de faire les commandes arpspoof et tcpdump
    && usermod -aG sudo Eve \
    # automatisation afin d'afficher le message d'introduction
    && echo "clear && cat intro.txt" >> /home/Eve/.bashrc

# ajout du message d'introduction
ADD intro.txt /home/Eve/intro.txt
RUN chmod 755 /home/Eve/intro.txt \
    && ip addr

USER Eve
WORKDIR /home/Eve

CMD ["bash"]
