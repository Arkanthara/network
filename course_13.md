plus alpha est proche de 1, plus on aura tendance à suivre les modifications... Plus alpha est proche de 0, plus 
Permet de faire des calculs stocastiques récursifs....



Variance de RTT permet de déterminer la valeur du temporisateur qu'on choisit....
Variance = D
D = alpha D + (1 - alpha) ||RTT-M||

valeur du temporisateur = RTT + 4D

reste 1% -> 1% de transmissions en trop

C'est mieux d'attendre avant de retransmettre, plutôt que de trop retransmettre


## Persistance

autre temporisateur: temporisateur de persistance.... Après expiration, il transmet un segment pour connaître la taille de la fenêtre du receveur.


# Fenêtre
16 bits... Pas beaucoup... -> on peut agrandir le champ fenêtre jusqu'à 14 bits...


Feedback: 0 -> augmente la charge (soit $\sumx_i(t) \lt y$

ui: mise à jour du i ème utilisateur... On ne connaît que nous même et le trafic global via le feedback...


veut déterminer l'efficacité... On veut utiliser au maximum la charge du réseau

implémentation distribuée: ui dépend seulement de xi et y

équité: tout le monde transmet le même nombre d'informations

index de Jain (Jain's index)

F(x) = \frac{(\sum x_i)²}{n \sum x_i²}

changement d'échelle: par exemple: passer de km/h en m/s
F est invariante par changement d'échelle.

mécanisme multiplicatif:

$x_0(t) = b_I \begin{bmatrix} x_1 \\ x_2 \end{bmatrix}$

modification additive: si c'est +, cela augmente l'équité, si c'est -, cela diminue l'équité...
Mécanisme additif pour augmenter le flux, et mécanisme multiplicatif pour diminuer le flux...


Pour converger vers une solution équitable, F(x(t)) -> 1

a = 0: équité constante... a > 0 équité augmente...

Taux d'erreur dans communication wifi est de 40%. En wifi, quand on a une erreur, il faut tout de suite retransmettre car le réseau n'est pas congestionné, c'est à cause des interférences.... Il faut donc penser à adapter TCP pour sans fil, sinon on peut avoir des problèmes...


# Modèle client serveur

Tous les membres du réseau peuvent être client ou serveur, demander un service ou réaliser un service


Middleware: niveau d'abstraction plus grand du réseau: on a une couche middleware donnant des services permettant de se connecter à un serveur...
C'est un logiciel entre le network et les applications...

Service a un niveau de spécification plus élevé qu'un logiciel (...???)

Serveurs itératifs
mode connecté
mode non connecté

Serveurs parallèles
mode connecté
mode non connecté


RPC temps d'exécution plus long
Attention ! Contexte différent !!"
