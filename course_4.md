P(2 trames) = $\frac{T}{T + \frac{1}{\lambda}}$ A apprendre par coeur !!!!! C'est la probabilité que au temps T, 2 se mette à transmettre...

probabilité de collision: $\frac{T}{T + \frac{1}{\lambda T}}$ (C'est la probabilité qu'au temps t, t2 se mette à transmettre)

trame transmisse par unité de temps: $\frac{\lambda}{1 + \lambda T}$

Probabilité trame soit transmise sans collision: $(\frac{1}{\lambda T + 1})^{k - 1}e^{-\lambda T (k - 1)}$

# Réseaux locaux

## Ethernet

Topologie en étoile: les pcs sont connectés au hub central par une liaison point à point...

Pour se connecter: on a besoin d'un tranceiver

Transmission: préambule: sert à syncroniser: B va mettre son horloge pour être à la bonne fréquence

Collision: A transmet, mais écoute et s'il entend A + quelquechose, c'est qu'il y a eu une collision....

Pour détecter une collision, il faut que la taille de la trame divisée par le débit soit plus grand que le temps de propagation.


