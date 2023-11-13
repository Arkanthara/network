Les stations sont asynchrones....

On passe toujours par les identifiants les plus petits...

802.1D
racine
distance
id

Port désactivé uniquement pour les trames de DONNÉES mais pas pour les trames déterminant qui est le noeud racine...

p194 importante !!!

Message Priority Vector
(pas stopé par port coupé)
racine
distance
id envoyeur
id port reçu

Sur chacun des ports, il faut identifier la priorité...

PPV mémorisation du message
racine
distance
id
port sur lequel l'autre transmet.
port sur lequel on transmet

RPV
racine
distance
id

PPV: message reçu (présent pour chaque port)
RPPV: information pour chaque port
DPV: min des RPPV
MPV: Message envoyé

DPV: min RPVs

Envoie MPV
Reçoit PPV
RPPV: mis à jour information pour chaque port
DPV: min de tous... DPV c'est le message qu'on a envoyé

Port activé si ce qui transmet est plus petit que son RPPV (Si DPV convertit en MPV est plus petit que RPPV)
Si ce qu'il envoie est  plus petit que ce qu'il envoie, il active le port, sinon, il le désactive...

Attention ! Il faut faire la mise à jour, puis le minimum des RPPV !!!

### MAC
Plusieurs types d'adresse MAC
L'atribution est gérée par IEEE

Format EUI: adresse MAC peut être attachée à des applications et pas à la carte réseau...

Réseau virtuel: perpet de séparer un réseau en 2, mais virtuellement....??

802.1Q: ajouté un ID de protocole et une étiquette possédant un id de VLAN...
