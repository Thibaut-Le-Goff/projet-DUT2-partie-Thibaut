<center><h2>Compte rendu de projet, partie de Le Goff Thibaut</h2></center>

La partie de Thibaut consiste à créer des épreuves afin d'alimenter le site web créé par Vincent.

Les épreuves ainsi créées ont aussi servi à présenter la spécialisation Cybersécurité du BUT R&T. 

**<ins>Première épreuve :</ins>**\
Cette épreuve consiste à décoder le code ASCII :

```shell
4c 65 20 66 6c 61 67 20 64 65 20 63 65 74 74 65 20 65 70 72 65 75 76 65 20 65 73 74 3a 20 40 24 63 49 7c 
```

Solution :
```shel
root@debtiti:/home/titi# echo 4c 65 20 66 6c 61 67 20 64 65 20 63 65 74 74 65 20 65 70 72 65 75 76 65 20 65 73 74 3a 20 40 24 63 49 7c | xxd -r -p
Le flag de cette epreuve est: @$cI|root@debtiti:/home/titi# 
```

**<ins>Première épreuve :</ins>**\

**<ins>Première épreuve :</ins>**\
Le principe de l'épreuve consiste à chercher un document .txt qui contient le flag.\
Cette épreuve se déroule dans un conteneur docker.

<ins>Comment l'épreuve a été créée :</ins>

Cette épreuve a été construite à partir d'un conteneur issu d'une image qui a été créé à l'aide de ce Dockerfile :

``` dockerfile
FROM ubuntu:latest

RUN apt-get update -y && apt-get upgrade -y && useradd -ms /bin/bash Lapinou

USER Lapinou
WORKDIR /home/Lapinou

CMD ["bash"]
```
Une fois dans le conteneur, les dossiers qui y ont était crées sont :

```shell
Lapinou@lapin:~$ ls   
Desktop  Documents  Downloads  Music  Pictures  Public  Templates  Videos
```
Le flag se cache dans /Pictures/terrier/flag.txt et un piège a été placé en /Documents/solutions-TP/TP-magnetisme :

```bash
Lapinou@lapin:~$ cd Documents/
Lapinou@lapin:~/Documents$ ls
solutions-TP
Lapinou@lapin:~/Documents$ cd solutions-TP/
Lapinou@lapin:~/Documents/solutions-TP$ cat TP-magnetisme 
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠛⠛⠛⠋⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠙⠛⠛⠛⠿⠻⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⡀⠠⠤⠒⢂⣉⣉⣉⣑⣒⣒⠒⠒⠒⠒⠒⠒⠒⠀⠀⠐⠒⠚⠻⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⡠⠔⠉⣀⠔⠒⠉⣀⣀⠀⠀⠀⣀⡀⠈⠉⠑⠒⠒⠒⠒⠒⠈⠉⠉⠉⠁⠂⠀⠈⠙⢿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠔⠁⠠⠖⠡⠔⠊⠀⠀⠀⠀⠀⠀⠀⠐⡄⠀⠀⠀⠀⠀⠀⡄⠀⠀⠀⠀⠉⠲⢄⠀⠀⠀⠈⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⠋⠀⠀⠀⠀⠀⠀⠀⠊⠀⢀⣀⣤⣤⣤⣤⣀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠜⠀⠀⠀⠀⣀⡀⠀⠈⠃⠀⠀⠀⠸⣿⣿⣿⣿
⣿⣿⣿⣿⡿⠥⠐⠂⠀⠀⠀⠀⡄⠀⠰⢺⣿⣿⣿⣿⣿⣟⠀⠈⠐⢤⠀⠀⠀⠀⠀⠀⢀⣠⣶⣾⣯⠀⠀⠉⠂⠀⠠⠤⢄⣀⠙⢿⣿⣿
⣿⡿⠋⠡⠐⠈⣉⠭⠤⠤⢄⡀⠈⠀⠈⠁⠉⠁⡠⠀⠀⠀⠉⠐⠠⠔⠀⠀⠀⠀⠀⠲⣿⠿⠛⠛⠓⠒⠂⠀⠀⠀⠀⠀⠀⠠⡉⢢⠙⣿
⣿⠀⢀⠁⠀⠊⠀⠀⠀⠀⠀⠈⠁⠒⠂⠀⠒⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⢀⣀⡠⠔⠒⠒⠂⠀⠈⠀⡇⣿
⣿⠀⢸⠀⠀⠀⢀⣀⡠⠋⠓⠤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠈⠢⠤⡀⠀⠀⠀⠀⠀⠀⢠⠀⠀⠀⡠⠀⡇⣿
⣿⡀⠘⠀⠀⠀⠀⠀⠘⡄⠀⠀⠀⠈⠑⡦⢄⣀⠀⠀⠐⠒⠁⢸⠀⠀⠠⠒⠄⠀⠀⠀⠀⠀⢀⠇⠀⣀⡀⠀⠀⢀⢾⡆⠀⠈⡀⠎⣸⣿
⣿⣿⣄⡈⠢⠀⠀⠀⠀⠘⣶⣄⡀⠀⠀⡇⠀⠀⠈⠉⠒⠢⡤⣀⡀⠀⠀⠀⠀⠀⠐⠦⠤⠒⠁⠀⠀⠀⠀⣀⢴⠁⠀⢷⠀⠀⠀⢰⣿⣿
⣿⣿⣿⣿⣇⠂⠀⠀⠀⠀⠈⢂⠀⠈⠹⡧⣀⠀⠀⠀⠀⠀⡇⠀⠀⠉⠉⠉⢱⠒⠒⠒⠒⢖⠒⠒⠂⠙⠏⠀⠘⡀⠀⢸⠀⠀⠀⣿⣿⣿
⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠑⠄⠰⠀⠀⠁⠐⠲⣤⣴⣄⡀⠀⠀⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀⠀⠀⢠⠀⣠⣷⣶⣿⠀⠀⢰⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠁⢀⠀⠀⠀⠀⠀⡙⠋⠙⠓⠲⢤⣤⣷⣤⣤⣤⣤⣾⣦⣤⣤⣶⣿⣿⣿⣿⡟⢹⠀⠀⢸⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠀⠀⠀⠑⠀⢄⠀⡰⠁⠀⠀⠀⠀⠀⠈⠉⠁⠈⠉⠻⠋⠉⠛⢛⠉⠉⢹⠁⢀⢇⠎⠀⠀⢸⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣀⠈⠢⢄⡉⠂⠄⡀⠀⠈⠒⠢⠄⠀⢀⣀⣀⣰⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⢀⣎⠀⠼⠊⠀⠀⠀⠘⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠉⠢⢄⡈⠑⠢⢄⡀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁⠀⠀⢀⠀⠀⠀⠀⠀⢻⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣀⡈⠑⠢⢄⡀⠈⠑⠒⠤⠄⣀⣀⠀⠉⠉⠉⠉⠀⠀⠀⣀⡀⠤⠂⠁⠀⢀⠆⠀⠀⢸⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣄⡀⠁⠉⠒⠂⠤⠤⣀⣀⣉⡉⠉⠉⠉⠉⢀⣀⣀⡠⠤⠒⠈⠀⠀⠀⠀⣸⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣶⣶⣤⣤⣤⣤⣀⣀⣤⣤⣤⣶⣾⣿⣿⣿⣿⣿
```
Source du dessin en ASCII : \
https://emojicombos.com/troll-face-ascii-art 

Une fois, cela fait, le conteneur a été transformé en image :

```shell
titi@Ordititi:~$ docker container ls -a
CONTAINER ID   IMAGE        COMMAND     CREATED         STATUS         PORTS     NAMES
f2be3f0db8a1   epreuvectf   "/bin/sh"   8 minutes ago   Up 8 minutes             blissful_carson
titi@Ordititi:~$ docker commit blissful_carson
sha256:edd6d977639525dff00b1d94425690fc4ba04414f0c9c4569404578819b4e5bb
```
L'image ainsi créée ressemble à cela :

```shell
titi@Ordititi:~$ docker image ls -a
REPOSITORY   TAG       IMAGE ID       CREATED          SIZE
<none>       <none>    edd6d9776395   11 seconds ago   108MB
epreuvectf   latest    a49e7919397f   11 minutes ago   108MB
ubuntu       latest    14119a10abf4   2 months ago     72.8MB
```

Ensuite cette image à était renommée :
```shell
titi@Ordititi:~$ docker tag edd6d9776395 ctfiutbeziers/lapinou
titi@Ordititi:~$ docker image ls -a
REPOSITORY               TAG       IMAGE ID       CREATED          SIZE
ctfiutbeziers/lapinou    latest    edd6d9776395   25 minutes ago   108MB
epreuvectf               latest    a49e7919397f   37 minutes ago   108MB
ubuntu                   latest    14119a10abf4   2 months ago     72.8MB
```

<ins>Réalisation de l'épreuve :</ins>\
Pour invoquer le conteneur il faut exécuter les fichier epreuve1-lapin.sh qui contient :

```bash
#!/bin/bash

docker pull ctfiutbeziers/lapinou && docker run -it --name lapin --hostname lapin ctfiutbeziers/lapinou
```

Après que le fichier ait fini son exécution nous arrivons sur le terminal de l'utilisateur Lapinou avec un message d'introduction afin d'expliquer l'épreuve :

```sheel
Lors de ce CTF, vous allez devoir chercher un fichier, son nom est 'flag', c'est un txt, bonne chance !
Lapinou@lapin:~$ 
```
La solution de l'épreuve est :
```
Lapinou@lapin:~$ find -name "*.txt"
./Pictures/terrier/flag.txt
Lapinou@lapin:~$ cat ./Pictures/terrier/flag.txt
```
La commande cat donne ainsi :
```

                           ____     ____
                          /'    |   |    \
                        /    /  |   | \   \
                      /    / |  |   |  \   \
                     (   /   |  """""  |\   \       
                     | /   / /^\    /^\  \  _|           
                      ~   | |   |  |   | | ~
                          | |__O|__|O__| |
                        /~~      \/     ~~\
                       /   (      |      )  \
                       /,   \____/^\___/'   \  
                  q&&&p / -____-|_|_|-____-\  .-^-.
               __*+*+*+*___/~~~~\___/~~~~\__.'=^=^='.
              /|x=X=X=X=x |     |   |     |/=^=^=^=^=\
             / |px..---.q `^-^-^' .-~-.^-^:^=^=^=^=^=^;
            /  ,`._`-.|_ `.     .'~~*~~'. |^=^=^=^=^=^|\
           / ,'._  `. `. |_\   /  *   *  \|^=^=^=^=^=^: \
          / /`-. `.  `. `.  :_:~*~~~*~~~*~;\=,-*))`*-,/  \
         /_:_____._`.__`._\_;_|___*___*___|/*__((*___*'\__\
         |                                                |
         |================================================|
         |~~(~~*~~(~~*~=       Flag:       =~*~~(~~*~~(~~*|
         |~(~~*~~(~~*~~= 03uF$_En_(h0{o|@t =*~~(~~*~~(~~*~|
         |(~~*~~(~~*~~(=====================~~(~~*~~(~~*~~|
         |================================================|
         |________________________________________________|
```
Source du dessin en ASCII : \
https://www.asciiart.eu/animals/rabbits \
https://asciiart.website/index.php?art=holiday/easter
