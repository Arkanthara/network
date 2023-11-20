# Protocoles de la couche 3

protocole IP

Adresses IPv4


Adresse MAC routable seulement localement...

Utilisation de table de routage pour savoir oû envoyer les données...

Les routeurs font jamais de broadcast en IPv4 ou en IPv6 car il y a beaucoup trop d'adresses

Comment trouver le chemin ?

On a des classes d'adresse... Il faut regarder l'entête de l'adresse (bits de poid fort...)

2 puissance 10 c'est 1024...

Problème: trop de machines comparé au nombre d'adresses disponibles...
-> agrandi la taille de l'adresse + nouvelles fonctionnalités


On est au niveau 3 du modèle OSI -> routage des données

Protocole IP: protocole sans connexion: il n'y a pas de connection entre A et B... Les paquets arrivent quand ils veulent... Différent du protocole TCP qui est avec connexion: A demande à B s'il peut lui envoyer des données etc...

On donne les 128 bits, puis la longueur du préfixe...

Il y a une association qui s'appelle IANA qui permet de donner des adresses...

Certaines plages d'adresses ont des fonctions particulières: 

Quand il y a beaucoup de 0, on peut regrouper tous les 0 ensemble et mettre 2 points à la place... On peut le faire qu'une seule fois car sinon on ne sait plus combien de 0 ajouter par le :

Préfixe: /3

Global unicast: adresse qu'on va pouvoir rooter à l'échelle global et on n'a qu'un seul déstinataire...
Adresses locales

Link-local unicast: joue le même rôle que les adresses MAC (Car dans la vision d'ipv6, on n'utilise plus MAC...)


On fait du routage hiérarchique pour les adresses global unicast...

Les bits de point fort permettent de définir la région....

Comment faire pour avoir sa propre adresse ???

On peut avoir des adresses fixes (routeur par exemple, car il faut pouvoir se connecter au routeur et donc connaître son adresse qui doit ne pas trop changer...

Obtenir une adresse: on demande au routeur de nous attribuer une adresse ?
