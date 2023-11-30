

Paquet ECHO: doit être retourné le plus rapidement possible par les routeurs recevant le paquet... Permet de mesurer le coùt de la ligne...


paquet d'état des liens:
- identité
- num sequence
- âge 
- voisins ???

Distribution de paquets d'état de lien on les distribue par innondation...


OSPF: protocole de référence calculant .....................



# TCP/IP (couche transport)

Liaison bout à bout: paquet pas modifié entre les 2 points..........???...


UDP fonctionne en mode non connecté
TCP fonctionne en mode connecté
protocole avec connexion
protocole sans connexion

TCP -> couche transport: fiable orienté connection -> vérifier que les données sont bien reçues, contrôle de flux, etc... -> fiabilise les données...
UDP -> couche transport: pas fiable, pas contrôle de flux, pas de connexion...

UDP -> paquets
TCP -> segments

!!!! On se connecte au numéro de port !!!!

Décalage: permet de fragmenter le segment car les stations n'ont pas forçément une taille de segment identique....

Synchronisation: S
Ackite: A
Reset: R
F: end connection


Pour établir une connection,, il faut créer des points de connection appelé socket.

Champ fenêtre va être utilisé pour faire du contrôle de flux... Dit combien la station qui a transmit ce champ peut recevoir de bytes dans son buffer. (Taille restante dans le buffer...)

Aquittent le nombre de bytes transmis....


C'est le récepteur qui contrôle le flux de l'émetteur.

On retarde les acquittements, car on espère aquitter plus de données d'un coup...

Remarques: le protocole de transmission utilisé en général est le protocole selective repeat....

fenêtre de congestion: permet de 'mesurer' si les paquets passent ou ne passent pas... Fenêtre de congestion multipliée par 2 à chaque fois qu'un aquittement est reçu...


On choisit un seuil ... à chaque fois que le temporisateur expire, on fixe le seuil à la moitié de la valeur du temporisateur...

Réception de 3 accusés de réception avec le même numéro x après avoir envoyé des paquets de plus grand numéro de séquence  ->  retransmission de x

On recommence au seuil si on reçoit 3 aquittements.... si c'est le temporisateur qui expire, on recommence depuis 0 la taille de la fenêtre de congestion
