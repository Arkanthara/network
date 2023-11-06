# Codage convolutif

Dépend de l'histoire de ce qu'on a déjà transmit.
Les turbo codes sont sur le principe du codage convolutif.

chaque chemin dans le treillis correspond à une séquence de bits transmise/reçue et inversement.
On se rend compte des erreurs si on n'a pas de chemin dans le treillis possible...
Parcours du treillis pas exponnentiel grâce à l'algorithme de Viterbi
Distance de Hamming: nombre de bits différents...

Il y a à chaque fois 2 arrêtes qui mènent vers un noeud du treillis
Quand on arrive à un noeud par 2 chemins différents, on oublie la distance de Hamming la plus grande...

Le chemin qui reste est appelé le chemin survivant.

Complexité linéaire de l'algorithme
Permet de corriger les erreurs de transmission lorsqu'elles ne sont pas en block...
Succession d'erreurs en block pouvant former des chemins possibles dans le treillis.

-> changement de mode d'envoi des bits: on envoie les bits par colonne, comme cela, lors des perturbations, les bits erronnés ne vont pas être en block...

Les performances dépendent du nombre d'état de l'automate...
transmet 3 bits pour transmettre 2 bits...... vs transmet 2 bits pour transmettre 1 bit par unité de temps -> débit Mbits/sec doublé......??..

# Interconnection des réseaux 2 LAN
p175 importante !

rôle de switch: faire communiquer 2 domaines de collision (un domaine de collision est lorsqu'une station peut entrer en collision avec une autre station...
Processus de routage: trouver un chemin entre a et b

Ici, on va faire du routage au niveau 2 (c'est ce qu'on appelle commutation au niveau 2...)

Switch = commutateur
On a également des répéteurs. Ils fonctionnent au niveau de la couche physique.

Hub: opèrent également au niveau de la couche physique. C'est une boite qui relie tout le monde à tout le monde...
Bridge/pont: permet l'interconnexion de LAN ou segment de LAN maintenant, plus trop de distrinctions entre pont et switch...
pont:
ll.receive(trame)
    if trame.type = ctrl ....
    if trame.type = donnée -> regarder l'adresse du destinataire et décider du port sur lequel transmettre la trame.

hub: est au niveau de la couche physique vs pont: est au niveau 2

Pont: fait du filtrage (il va essayer de ne pas envoyer la trame à tout le monde, mais uniquement au réseau possédant la station) et de la retransmission

table de routage:
station port heure
c1       1
d0       2
b2       1
etc..
On a besoin de l'adresse LAN (Local Area Network) des stations, l'interface de pont qui y conduit, l'heure d'actualisation des données.

Initialement, la table du pont est vide... Elle peut se remplir au fur et à mesure suivant les trames reçues.
Si la destination n'est pas dans la table, on transmet sur tous les ports

Switch: permet de connecter directement des ordinateurs. Ne transmet que à la personne concernée, à la différence d'un hub.

Phase de découverte: si l'adresse de destination n'apparait pas dans sa table de routage, il transmet sur tous les ports, sauf sur celui duquel il a reçu la trame.

Adresse broadcast: transmission sur tous les ports... (sauf celui duquel le message arrive

On crée une boucle... Dans ces conditions si on a un broadcast, on mémorise le numéro de trame et si on la revoie, on ne la retransmet pas... Mais si on ne connait pas le destinataire ??? On a un soucis...
boucles -> difficulté dans les phases de découverte
redondance -> fiabilité: si le chemin du haut est bloqué, on passe par en bas...

On aimerait donc avoir une topologie donnée par un graphe, et la transformer en arbre pour enlever les cycles...
Pour déconnecter 2 noeuds du graphe, il faut désactiver les ports des switch...
-> on exécute un protocole qui définit si un port doit être actif ou inactif...

Spanning tree (arbre de couverture)
On aimerait extraire un arbre de notre graphe, et on aimerait que l'arbre soit un arbre de recouvrement...

On va donc activer et désactiver d'une manière dynamique les ports pour avoir un arbre de recouvrement...

2phases: déterminer quel sera la station noeud racine (quel est le centre du monde...), puis  trouver les plus courts chemins de la racine vers les autres noeuds...
blanc: chemin le plus court en passant par ce port pour atteindre la racine
noir: détermine que c'est un port feuille...
blanc: port racine: plus court chemin pour atteindre la racine
noir: port feuille: port ammenant 

au début, chaque pont va se dire qu'il est la racine.
puis il va transmettre sur chacun de ses ports pour dire qu'il est la racine
Enfin, c'est celui qui a l'identifiant le plus petit qui sera la racine...
