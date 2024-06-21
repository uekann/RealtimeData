package org.lib.stock;

public class Stock {
    public String name;
    public double open;
    public double high;
    public double low;
    public double close;

    public Stock(){}

    public Stock(String name, double open, double high, double low, double close) {
        this.name = name;
        this.open = open;
        this.high = high;
        this.low = low;
        this.close = close;
    }

    @Override
    public String toString() {
        return String.format("%s, %.2f, %.2f, %.2f, %.2f", name, open, high, low, close);
    }

    public static Stock fromString(String line) {
        String[] tokens = line.split(",");
        return new Stock(tokens[0], Double.parseDouble(tokens[1]), Double.parseDouble(tokens[2]), Double.parseDouble(tokens[3]), Double.parseDouble(tokens[4]));
    }

    public static class TimedStock{
        public long timestamp;
        public Stock stock;

        public TimedStock(){}

        public TimedStock(long timestamp, Stock stock){
            this.timestamp = timestamp;
            this.stock = stock;
        }

        public static TimedStock fromString(String line){
            Stock stock = Stock.fromString(line);
            return new TimedStock(System.currentTimeMillis(), stock);
        }
    }
}