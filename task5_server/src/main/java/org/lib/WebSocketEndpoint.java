package org.lib;

import org.java_websocket.server.WebSocketServer;
import org.java_websocket.WebSocket;
import org.java_websocket.handshake.ClientHandshake;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import org.apache.flink.streaming.api.functions.sink.SinkFunction;

import java.net.InetSocketAddress;
import java.util.Collections;
import java.util.HashSet;
import java.util.Set;

public class WebSocketEndpoint extends WebSocketServer{
    private static final Set<WebSocket> clients = Collections.synchronizedSet(new HashSet<>());
    private static final Logger logger = LogManager.getLogger(WebSocketEndpoint.class);

    public WebSocketEndpoint(InetSocketAddress addr){
        super(addr);
        logger.info("WebSocketEndpoint created : {}", addr.toString());
    }

    @Override
    public void onOpen(WebSocket conn, ClientHandshake handshake){
        clients.add(conn);
        logger.info("Opened connection to {}", conn.getRemoteSocketAddress());
    }

    @Override
    public void onClose(WebSocket conn, int code, String reason, boolean remote){
        clients.remove(conn);
        logger.info("Closed connection to {}", conn.getRemoteSocketAddress());
    }

    @Override
    public void onMessage(WebSocket conn, String message){
        logger.info("Received message from {}: {}", conn.getRemoteSocketAddress(), message);
    }

    @Override
    public void onError(WebSocket conn, Exception ex){
        logger.error("Error occurred on connection to {}", conn.getRemoteSocketAddress());
        ex.printStackTrace();
    }

    @Override
    public void onStart(){
        logger.info("WebSocket server started");
    }

    public static void sendMessage(String message){
        for(WebSocket client : clients){
            client.send(message);
        }
        logger.info("Sent message to all clients: {}", message);
    }

//    public static class WebScoketSink implements SinkFunction<String>{
//        @Override
//        public void invoke(String value, Context context) throws Exception{
//            WebSocketEndpoint.sendMessage(value);
//        }
//    }
}