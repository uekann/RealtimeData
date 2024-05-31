"use client";

import React, { useEffect, useState } from 'react';
import { Bar } from 'react-chartjs-2';
import { Chart as ChartJS, CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend } from 'chart.js';
import { ChartData, ChartOptions } from 'chart.js';
import ChartComponent from "../components/ChartComponent"
import TableComponent from "../components/TableComponent"

const SOCKET_SERVER_URL = process.env.WEB_SERVER_IP
const SOCKET_SERVER_PORT = process.env.WEB_SERVER_PORT
const socketServerAddress = `http://127.0.0.1:3030`;


ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend);

const options: ChartOptions<'bar'> = {
  responsive: true,
  plugins: {
    legend: {
      position: 'top',
    },
  },
};

const Home: React.FC = () => {
  const [stockInfo, setStockInfo] = useState<object>({});
  const [records, setRecords] = useState<object[]>([]);

  useEffect(() => {
    console.log(`Connecting to WebSocket server at ${socketServerAddress}`);

    const socket = new WebSocket(socketServerAddress);

    socket.onmessage = (event) => {
      console.log('Message received:', event.data);
      setStockInfo(JSON.parse(event.data)["info"]);
      setRecords(JSON.parse(event.data)["records"]);
    }
  }, []);

  return (
    <div>
      <ChartComponent stockInfo={stockInfo} />
      <TableComponent records={records} />
    </div>
  );
};

export default Home;
