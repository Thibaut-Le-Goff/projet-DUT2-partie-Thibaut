FROM debian:stretch
RUN apt-get update -y \
    && apt-get upgrade -y \
    && apt-get install -y openssh-server \
    && apt-get install -y python3 \
    && apt-get install -y python3-pip \
    && mkdir /var/run/sshd \
    # Création de l'utilisateur Bob
    && useradd -ms /bin/bash Bob \
    # Donne le mot de passe "Bob" à Bob
    && echo Bob:Bob | chpasswd \
    # ajout de Bob dans le groupe sudo 
    && usermod -aG sudo Bob

    #Dans un bash:
    #&& ssh-keygen -b 1024 -P abc123 -f /root/.ssh/id_rsa \
    #&& cat /root/.ssh/id_rsa > /home/Bob/clef_pub_Bob \
    #&& chmod 600 /home/Bob/clef_pub_Bob \
    #&& scp John@172.10.0.1:/home/clef_pub_Bob
ADD dif.sh /dif.sh
RUN chmod 755 /dif.sh \
    && echo "cd /home/Bob && su Bob" >> /home/Bob/.bashrc

CMD ["./dif.sh", "Good luck", "abc123"]
