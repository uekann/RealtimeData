package org.lib.stock;

import org.apache.flink.api.java.tuple.Tuple2;
import org.lib.stock.Stock;
import org.lib.stock.Stock.TimedStock;

import java.util.*;


public class StockInfo {

    public static class CloseDatas{
        public double min;
        public double max;
        public double avg;
        public double std;

        public CloseDatas(double min, double max, double avg, double std){
            this.min = min;
            this.max = max;
            this.avg = avg;
            this.std = std;
        }

        public CloseDatas(){}
    }

    public TimedStock[] stocks;
    public Tuple2<String, CloseDatas>[] closeDatas;

    public StockInfo() {
    }

    public StockInfo(TimedStock[] stocks, Tuple2<String, CloseDatas>[] closeDatas) {
        this.stocks = stocks;
        this.closeDatas = closeDatas;
    }

    public String toJson(){
        StringBuilder sb = new StringBuilder();
        sb.append("{");
        sb.append("\"records\": [");
        boolean first = true;
        for (TimedStock timedStock : stocks) {
            if (first) {
                first = false;
            } else {
                sb.append(", ");
            }
            Stock stock = timedStock.stock;
            sb.append("{");
            sb.append("\"timestamp\": ").append(timedStock.timestamp).append(", ");
            sb.append("\"data\": {");
            sb.append(String.format("\"stock\": \"%s\", ", stock.name));
            sb.append(String.format("\"open\": %.2f, ", stock.open));
            sb.append(String.format("\"high\": %.2f, ", stock.high));
            sb.append(String.format("\"low\": %.2f, ", stock.low));
            sb.append(String.format("\"close\": %.2f", stock.close));
            sb.append("}");
            sb.append("} ");
        }
        sb.append("], ");
        sb.append("\"info\": {");
        first = true;
        for (Tuple2<String, CloseDatas> entry : closeDatas) {
            if (first) {
                first = false;
            } else {
                sb.append(", ");
            }
            CloseDatas datas = entry.f1;
            sb.append("\"").append(entry.f0).append("\": {");
            sb.append(String.format("\"min\": %.2f, ", datas.min));
            sb.append(String.format("\"max\": %.2f, ", datas.max));
            sb.append(String.format("\"avg\": %.2f, ", datas.avg));
            sb.append(String.format("\"std\": %.2f", datas.std));
            sb.append("} ");
        }
        sb.append("}");
        sb.append("}");
        return sb.toString();
    }
}
