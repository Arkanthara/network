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
