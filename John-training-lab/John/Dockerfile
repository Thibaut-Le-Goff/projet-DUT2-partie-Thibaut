FROM debian:stretch
RUN apt-get update -y \
    && apt-get upgrade -y \
    && apt-get install -y openssh-server \
    && apt-get install -y python3 \
    && apt-get install -y python3-pip \
    && apt-get install -y sudo \
    # Création de l'utilisateur John
    && useradd -ms /bin/bash John \
    # Donne le mot de passe "John" à John
    && echo John:John | chpasswd \
    # ajout de John dans le groupe sudo
    && usermod -aG sudo John

USER John
WORKDIR /home/John

CMD ["bash"]