# MAC: Medium Access Control
- 3 types de trames:
  - trames de gestion
  - trames de contrôle
  - trames de données

(CSMA/CD) Mécanisme d'accès: écouter avant d'émettre... Pas possible si stations pas dans le rayon d'émission...
-> trames RTS inventées pour pallier à ce problème...

Protocole de base est de type CSMA: on suppose que chaque station peut entendre si les autres stations sont en train d'émettre...

### Gestion des temps d'attente
Si on ne peut pas transmettre tout de suite, on va attendre un temps DIFS, puis on va attendre des slots de temps (le nombre de slots à attendre est aléatoire...)

#### Stations cachées...
Il existe des configurations dans lesquelles une stations ne perçoit pas le signal d'une station émettrice...

RTS/CTS: est-ce que je peux transmettre (Request To Send)? Oui tu peux...(Clear To Send) 
Vulnérabilités avec RTS et CTS -> trames beaucoup plus petite...

Station 2 attend l'aquittement... Il ne faut pas qu'elle ne voie pas l'acquittement...

Problème: récepteur utilise autant d'énergie lorsqu'il écoute que lorsqu'il transmet...

### RTS/CTS EIFS
Dans certaines configurations, une station perçoit que le canal est occupé sans pouvoir décoder les informations... Dans ces conditions, il attend un temps EIFS
permettant aux deux autres stations de finir leur échange RTS/CTS. Ainsi, lorsqu'il recommence à écouter, il détecte que le cannal est occupé. Puis, lorsque le canal
n'est plus occupé, il attend de nouveau un temps EIFS pour que l'acquittement puisse être échangé.

Dans certains articles, on peut lire que RTS/CTS est utiliser pour pallier au Hidden terminal problem. C'est faux !

#### Fragmentation

Si les données à transmettre sont trop volumineuses, la couche MAC va fragmenter les données...

Avantage: dès qu'il y a une erreur, toute les stations sont à égalité pour obtenir le canal

Ajout RTS/CTS -> protocole MACA
MACAW améiloration de MACA avec des temps d'attente etc...

# Mode PCF
Pas de mécanisme temps réel possible (On ne peut pas mettre des bornes sur la transmission)

BEACON: synchronisation des stations et fixe le temps

On switch entre le mode DCF et le mode PCF...

Au début de la période sans contention (CFP), on émet une trame BEACON pour sighifier le début du mode PCF (après avoir attendu un temps PIFS = Point-Inter Frame Space,
 SIFS < FIFS < DIFS). BEACON prioritaire par rapport à DIFS, mais pas prioritaire par rapport à SIFS (trames de contrôle).
p134 les différents temps d'attente sont variables pour fixer les priorités...

# Modulation des signaux
Regarder slides 141 et 142 du cours !!!
On a tendance à supprimer tout ce qui bouge en fonction du temps...
