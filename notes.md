# IPv6

IPv6 -> longeur 128 bits
premiers bits forment préfixe, qui est le numéro du réseau
bits restants forment interface id

## GUA
global unicast -> 2000::/3 à 3fff::/3
router datagrammes

### Allocation dynamique

1. interface génère un message __Routeur Sollicitation__ (RS) sur le __réseau local__ (link)
2. Routeur recevant message répond avec __Router Advertisement__ (RA)


#### RA
contient préfixe/longueur préfixe/info sur le link/passerelle par défaut

__Sont émis péridodiquement__

station peut:
- __stateless address configuration__ (SLAAC) (création  adresse avec préfixe du RA et adresse source du RA comme passerelle par défaut)
- Idem point 1 + contact __DHCP__ pour obtenir informations complémentaires (ex: DNS)
- RA suggère de contacter __DHCP__, passerelle par défaut est la source du RA

#### Remarque

RS et RA $\in$ protocole __Neighbour Discovery Protocol__ (NDP)

NDP propose également __Neighbourg Solicitation__ (NS) et __Neighbourg Advertisement__ (NA) (Remplacent protocole ARP de IPv4... NS: je connais l'adresse IPv6 mais pas MAC.  NA: c'est moi et mon MAC est...)

## ULA
unique local address -> fc00::/7 -> fdff::/7


## L-LUA
link-local unicast -> fe80::/10 à febf::/3

Utilisée __uniquement sur le link local__ (LAN)

__Chaque station possède une adresse 
multicast -> ff00::/8 à ffff::/8
