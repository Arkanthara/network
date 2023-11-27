# Adresse multicast


commutateur: cube
routeur: cylindre
 (p 18 cours)


!!!! Toujours donner la longueur du préfixe !!!!!!

ipV6 -> 33-33-xxxx... -> Adresse MAC

Les datagrammes link local sont filtrés automatiquement

OSPF: algorithme de routage possédant des noeuds spéciaux

Adresse SNM possède un préfixe comme donné dans le cours

p21

Chaque adresse correspond à un groupe multicast...
SNM: permet de filtrer les messages.... Permet de ne pas avoir à trop dépaquetter l'adresse..........???..??.. Filtrage niveau 2 -> on va beaucoup plus vite...

paquet couche ip: addresse destination, adresse source

Je connais l'adressse IP et je cherche l'adresse MAC
paquet ICMP


Protocole MLD:

Le routeur demande si sur un lien, une station appartient à un groupe

ff02::16 well-known -> correspond à une adresse de broadcast sur tous les routeurs du groupe....


La station dit qu'elle quitte le groupe: ff02:2

flow: associer un id à tous les datagrammes...


Donner la longueur du datagramme pour déterminer la longueur des données....

Next-header -> identifiant pour dire que les données ne doivent pas être transférer à la couche supérieure (Ex: attention, c'est un paquet ipv4 !)

Hop limit: décrémenté par chaque routeur ... Si égal à 0, alors destruction du message....

Pas de checksum...

IpSec
utilisé pour garantir la sécurité...
- authentification header
- encapsulating security blabla

AH
AH tunnel -> ip invariante
ESP -> crypte les données
ESP tunnel -> idem + ip

Routage par vecteur distance:
- taille du réseau limité à 15 hops... (nb d'arrêtres entre 2 noeuds)
-
-

On transmet le vecteur destination distance à tout le monde...

peer to peer: envoye au voisin directement....

Protocole implémentant cela: protocole RIPng

Mécanisme d'horizon partagé: si c va à A en passant par B, il n'envoie pas toutes les informations passant par B....
