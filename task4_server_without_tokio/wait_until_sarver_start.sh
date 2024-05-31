#!/bin/zsh

pwd
while ! grep -q "Listening" server.log; do
    sleep 1
done

#kill -s 0 -Kill $$