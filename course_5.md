Temps de propagation d'une trame = $\tau$

$\Rightarrow$ fenêtre de vulnérabilité = $2 \times \tau$

- détectent une collisiont

- émettent un signal de brouillage pour informer les autres stations qu'il y a eu des collisions

La durée détection de collision + réinitialisation du système $\lt \tau$

Cycles composés d'un temps d'attente exponentiel ($\lambda-exp$) + durée de l'envoi d'une trame

On peut ajouter un système où toutes les stations attendent que la trame est été acquitté (dès que la station remarque qu'on a arrêté de transmettre, on attends un temps fixe, puis on attend un temps aléatoire avant de transmettre)

#### Exponential backoff

Si les temps aléatoires choisi sont trop court, on risque d'avoir trop de collisions...

Si on a des collisions, on attend un temps d'attente 2 fois plus long....

Dans certains systèmes, les temps d'attente sont des slots de temps (on tire une variable aléatoire dans un intervale $x_{min}; x_{max}$, puis on fait la valeur obtenue fois le slot (période d'une durée déterminée...))

CSMA/CD: détection collision + (carrier sense): détection si quelqu'un parle

$kp(1 - p)^{k - 1}$: je peux choisir parmi k stations pour savoir laquelle va transmettre -> k fois probabilité que la station transmette (p) fois probabilité que les k-1 autres stations ne transmettent pas $(1 - p)^{k - 1}$

Il faut bien choisir p (si p = 1/k, on a des performances optimales)

Probabilité que le nombre de slots de contention soit j, c'est à dire le nombre de slots total avant émission correcte d'une trame $$A(1 - A)^{j - 1}$

Période de contention: dès qu'on n'envoie rien

(Regarder page 79 du cours !)

La durée moyenne d'un cycle est donnée par 1/A

P: durée d'émission d'une trame

2T: durée d'un slot

1/A: nombre de slots moyen

(Regarder page 80 du cours !)

Longueur des trames importante dans les performances du système !!!!!

Efficacité du canal: savoir si le canal est occupé...

#### Protocoles à jetons

Problème aloah/ethernet: 

Palier le problème: utilisation protocoles à jetons

Seule la trame qui dispose du jeton est autorisée à transmettre des données

On forme un anneau logique et les stations se transmettent le jeton. Quand la station a le jeton, elle peut transmettre

But: assurer que chaque station va pouvoir transmettre pendant $H_k$ secondes lorsqu'elle dispose du jeton

But déterminer $T_{TRT}$ (temps de rotation) en fonction du temps de transmission de chaque station ($H_k$).....
