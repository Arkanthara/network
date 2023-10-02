#### Matrices et autres notes

$P_{i, j}$ = probabilité de passer de l'état i à l'état j

$S_k(t)$: probabilité d'être à l'état k au temps t-1

$(S_0(t-1) \dots S_n(t-1))P = (S_0(t) \dots S_n(t))$

But: trouver $S(t) \limit_{\inf} P^{\inf}$

Protocole avec connection logique
    - Minimum: être d'accord sur les numéros de séquence
    - authentification
    - $\vdots$

TCP: orienté connection
UDP: pas orienté connection

Le RTT (Round Trip Time) varie. (C'est le temps qui s'écoule entre le moment ou un paquet est émis jusqu'à la réception de l'acquittement

#### Réseaux locaux

Réseau locaux:
- Réseau dispersé sur une petite distance
- Généralement privé
- Plus d'informations peuvent être obtenues car on a la gestion du réseau
-> faibles distances de communications et délais de transmission borné

Média de communication peut être:
- un câble partagé par les stations hôtes et transportant des signaux électriques
- des ondes électromagnétiques. On parle de radio-fréquence (RF) pour les ondes radios comprises entre 3KHz et 300GHz utilisés pour les radiocommunications.
Dans les 2 cas, on parle de liaison à diffusion. Les stations hôtes se partagent le même canal de communication. (Il y en a qu'un seul qui parle sur le réseau)

Le média de communication peut aussi être
- un ensemble de câbles reliant chaque station hôte à toutes les autres. Dans ce cas, on a un ensemble de liaisons point à poin.

##### Réseaux à diffusion ALOAH

Une famille de protocoles à diffusion très répandue est la famille des protocoles ALOHA: Structure de réseau en Hub ou en étoile...

Divison fréquentielle (FD) réserve des fréquences pour chaque radios

Solution: 2 fréquences: une pour la transmission de Ohaou

- station attend un temps aléatoire avec une distribution exponentielle
- station transmet la trame
- station attend un temps fixe. Si un acquittement positif est recu d'Ohau pendant l'attente la transmission est finie, sinon retour au point 1.

Protocole exécuté par Ohau est:
- attendre une trame
- si la trame est recue correctement, transmettre un acquittement positif
- retour en 1

Probabilité.
On a bernouli P(X = 0) = p = 1 - $\frac{\lambda}{n}$
P(X = 1) = 1 - p

On a X environ égal à $\lambda - exp$ quand n tend vers l'infini...

Si E(X) est le temps d'attente, alors P(E(X) | X > t_1) = P(attente)

Si une station attend depuis un temps t, le temps qui lui reste avant de retransmettre est toujours une loi exponentielle

Si on superpose 2 processus de poissons de paramètre $\lambda, \beta$, le processus résultant sera un processus de poisson de paramètre $(\lambda + \beta)$

On attend $\lambda - exp$

hypothèses:
- longueur trame: T
- stations saturées
- transmissions indépendantes
- distributions exponentielles de même paramètre $\lambda$

slot: si on atterit entre 2 slots, on attend un delay supplémentaire avant de transmettre:
-----|-----|------| -> on ne peut pas atterir entre les |: si on atterit entre les |, on attend pour arriver à un |.

G: numbre moyen de trames transmises par unité de temps de trame (nouvelles trames et retransmissions)
S: débit effectif (trames transmises avec succès)

Pour Aloha pur: $S = Ge^{-2G}$

$e^{-2G}: probabilité qu'il n'y ait pas de trames transmises pendant la période 2 (durée de l'interval) -> probabilité qu'il n'y ait pas de collisions....????

On a en moyenne G points par seconde...

------|--------|---|-------|----------|--|-----

-> probabilité que la trame 3 soit transmise sans collision: $lim \epsilon -> 0 [t - 1, t - \epsilon[ \union ] t + \epsilon, t + 1] = e^{-G} e^{-G} = e^{-2G}$

Pour slotted Aloha: $S = Ge^{-G}


