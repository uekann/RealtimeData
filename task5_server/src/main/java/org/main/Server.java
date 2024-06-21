/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.main;

import java.net.InetSocketAddress;

import org.apache.flink.api.common.time.Time;
import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.streaming.api.datastream.DataStream;
import org.apache.flink.streaming.api.functions.sink.SinkFunction;
import org.apache.flink.streaming.api.windowing.assigners.SlidingProcessingTimeWindows;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import org.lib.stock.Stock;
import org.lib.stock.Stock.TimedStock;
import org.lib.stock.StockInfo;
import org.lib.stock.ProcessAggregator;
import org.lib.WebSocketEndpoint;
//import org.lib.WebSocketEndpoint.WebScoketSink;

public class Server {

    public static void main(String[] args) throws Exception {
        final Logger logger = LogManager.getLogger(Server.class);

        final StreamExecutionEnvironment env = StreamExecutionEnvironment.getExecutionEnvironment();

        final String host_web = "localhost";
        final int port_web = 3030;
        final InetSocketAddress addr = new InetSocketAddress(host_web, port_web);
        WebSocketEndpoint server = new WebSocketEndpoint(addr);
        server.start();

        final String host_stream = "localhost";
        final int port_stream = 5000;
        DataStream<TimedStock> dataStream = env.socketTextStream(host_stream, port_stream)
            .map(TimedStock::fromString);

        dataStream
            .windowAll(SlidingProcessingTimeWindows.of(Time.seconds(30).toDuration(), Time.seconds(4).toDuration()))
//            .countWindowAll(30, 2)
            .process(new ProcessAggregator())
            .addSink(new SinkFunction<StockInfo>() {
                @Override
                public void invoke(StockInfo stockInfo) throws Exception {
                    WebSocketEndpoint.sendMessage(stockInfo.toJson());
                }
            });

        env.execute("task5_server");
    }
}