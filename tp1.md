# Pseudo-code for Stop-and-Wait protocol

busy = false
trame-number
trame-total-number

linklayer.handlerAlarm():
    hd.send(trame)
    reset(timer)

linklayer.send(data):
    if (!busy):
        trame = package(data + trame-number + MAC)
        hd.send(trame)
        busy = true
        start(timer)

linklayer.receive(answer):
    if (answer.type == NACK && trame-number == answer.ack.num):
        hd.send(trame)
        reset(timer)
    else if (answer.type == ACK && trame-number == answer.ack.num):
        stop(timer)
        trame-number ++
        busy = false
    else if (answer.type == data && trame-number == answer.trame.num):
        if (iscorrect(answer.trame.crc)):
            answer-2.type = NACK
            answer-2.ack.num = trame-number
            hd.send(answer-2)
        else:
            answer-2.type = ACK
            answer-2.ack.num = trame-number
            hd.send(answer-2)

# Pseudo-code for Go-Back-N

linklayer.send(buffer, N, option: trame-num):
    if (trame-num is given):
        num = trame-number-send(buffer, num)
        hd.send(buffer[num])
        for i in range(N):
            if (i != num):
                hd.send(buffer[i])
    else:
        for i in range(N):
            hd.send(buffer[i])

trame-number-send(buffer, num) -> return the indice of the buffer of trame where trame.num = num

trame-number-tampon(buffer) -> return the list of the numbers of trames in buffer

delete-trame-tampon(number) -> delete the trame where trame.number = number

refresh-tampon() -> if buffer is empty, put new trames in buffer.

correct-trame-number() -> verify that trame.number has a correct number (in a range [number - N; number + N]?) and not send by other...

linklayer.receive(trame):
    if (trame.type == ACK && trame.number in trame-number-tampon(buffer)):
        delete-trame-tampon(number)
        refresh-tampon()
    else if (trame.type == NACK && trame.number in trame-number-tampon(buffer)):
        linklayer.send(buffer, N)
    else if (trame.type == data && correct-trame-number()):
        if (iscorrect(trame.crc)):
            newtrame.type = ACK
            newtrame.ack.num = trame.number
            hd.send(newtrame)
        else:
            newtrame.type = NACK
            newtrame.ack.num = trame.number
            hd.send(newtrame)



Attention, il y a un système d'alarme !!!!!!!! Ajouter temporisateur !!!!!!!! On a un seul timer !!!!!!!



MAC: Medium Access Control C'est une sous-couche de la liaison de donnée pour accéder à ... C'est dans cette couche MAC qu'on a ALOAH, 802.3, les protocoles à jetons, 802.11 etc...
