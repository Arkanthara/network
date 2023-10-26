# Protocole à jeton (802.3)

Si le jeton tourne plus vite que le temps de rotation, soit elle passe le jeton, soit elle utilise ce temps pour envoyer une invitation (asynchrone).
Le trafic asynchrone possède un ordre...

# Wifi (802.11)
- atténuation rapide du signal
  - chouette si $\alpha$ = 2 car le signal se propage comme une sphère et la surface est 
  - Puissance reçue = puissance de l'émetteur / distance au carré
  - Puissance reçue forme un cercle: c'est isotropique...

- Limitation de la puissance émise $\rightarrow$ on ne veut pas vivre dans un micro-ondes...
- Consommation d'énergie
- Plus difficile d'assurer la sécurité des transmissions

Appartient à la couche 2, plus précisément à la couche Medium Access Control (MAC) de la couche 2

Dans ll: gobackN, stopandwait, ....
Dans MAC: écouter, transmettre...

802.3 topologie en bus
802.11

Mode ad-hoc: on n'a pas de point d'accès... Ex: discussion entre 2 ordinateurs... C'est par exemple le bluetooth...
Mode infrastructure: on a un point d'accès

BSS: basic service set

Scan passif: on attend une trame BEACON permettant la synchronisation de la station avec le point d'accès
Scan actif: émet une requêtre et attend une réponse du point d'accès

Pour faire un réseau ad-hoc: besoin de connaître les addresses MAC des correspondants...