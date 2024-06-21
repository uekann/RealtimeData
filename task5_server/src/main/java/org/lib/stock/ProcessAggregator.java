package org.lib.stock;

import org.apache.flink.api.java.tuple.Tuple2;
import org.apache.flink.streaming.api.windowing.windows.TimeWindow;
import org.apache.flink.streaming.api.windowing.windows.GlobalWindow;
import org.apache.flink.streaming.api.windowing.windows.Window;
import org.apache.flink.streaming.api.functions.windowing.ProcessAllWindowFunction;
import org.apache.flink.util.Collector;

import org.lib.stock.StockInfo;
import org.lib.stock.StockInfo.CloseDatas;
import org.lib.stock.Stock;
import org.lib.stock.Stock.TimedStock;

import java.util.*;

public class ProcessAggregator extends ProcessAllWindowFunction<TimedStock, StockInfo, TimeWindow> {

    @Override
    public void process(Context context, Iterable<TimedStock> stocks, Collector<StockInfo> out) {

        List<TimedStock> stockList = new ArrayList<TimedStock>();
        List<Tuple2<String, CloseDatas>> closeInfo = new ArrayList<Tuple2<String, CloseDatas>>();

        HashMap<String, List<Double>> stockClose = new HashMap<String, List<Double>>();

        for (TimedStock timedStock : stocks) {
//            stockInfo.addTimedStock(timedStock);
            stockList.add(timedStock);
            Stock stock = timedStock.stock;
            if (!stockClose.containsKey(stock.name)) {
                stockClose.put(stock.name, new ArrayList<Double>());
            }
            stockClose.get(stock.name).add(stock.close);
        }

        for (String key : stockClose.keySet()) {
            List<Double> close = stockClose.get(key);
            double min = Double.MAX_VALUE;
            double max = Double.MIN_VALUE;
            double sum = 0;
            for (double c : close) {
                if (c < min) {
                    min = c;
                }
                if (c > max) {
                    max = c;
                }
                sum += c;
            }
            double avg = 0;
            double std = 0;
            if (close.size() > 0) {
                avg = sum / close.size();
                for (double c : close) {
                    std += Math.pow(c - avg, 2);
                }
                std = Math.sqrt(std / close.size());
            } else {
                min = 0;
                max = 0;
            }
            CloseDatas closeDatas = new CloseDatas(min, max, avg, std);
//            stockInfo.setCloseData(key, closeDatas);
            closeInfo.add(new Tuple2<String, CloseDatas>(key, closeDatas));
        }

        stockList.sort(Comparator.comparingLong(a -> a.timestamp));
//        stockInfo.update();
        StockInfo stockInfo = new StockInfo(stockList.toArray(new TimedStock[0]), closeInfo.toArray(new Tuple2[0]));
        out.collect(stockInfo);
    }
}
