#!/bin/zsh

# server.logに"Error"か"Connection closed"という文字列が出るまで待つ
while true; do
    if grep -q "Error" server.log; then
        echo "Error found in server.log"
        break
    elif grep -q "Connection closed" server.log; then
        echo "Connection closed found in server.log"
        break
    fi
    sleep 1
done


export SERVER_PID=$(cat ./server.pid)
#kill $SERVER_PID

rm server.pid
rm server.log
echo "Server stopped"
kill -Kill $$