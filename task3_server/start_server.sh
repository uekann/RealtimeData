export $(cat ./.env)
echo "Starting server..."
cargo run --color=always --bin=task3_server --package=task3_server > ./server.log 2>&1 &

# バックグラウンドプロセスのPIDを取得
SERVER_PID=$!

# PIDをファイルに保存
echo $SERVER_PID > server.pid

echo "Server started with PID $SERVER_PID"
#kill -Kill $$