"use client"; // クライアントコンポーネントとして指定

import React from 'react';
import { Bar } from 'react-chartjs-2';
import { Chart as ChartJS, CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend, ChartOptions, ChartData } from 'chart.js';

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend);

const options: ChartOptions<'bar'> = {
    responsive: true,
    plugins: {
        legend: {
            position: 'top',
        },
    },
};
const ChartComponent = ({ stockInfo }) => {
    const dataLabels = ["min", "max", "avg", "std"]
    const dataColors: Record<string, string> = {
        "min": 'rgba(255, 99, 132, 0.8)',
        "max": 'rgba(54, 162, 235, 0.8)',
        "avg": 'rgba(255, 206, 86, 0.8)',
        "std": 'rgba(75, 192, 192, 0.8)',
    }

    const label = stockInfo ? Object.keys(stockInfo) : [];
    const datasets = stockInfo ? dataLabels.map((label) => {
        const data = Object.values(stockInfo).map((row) => row[label]);
        return {
            label: label,
            data: data,
            backgroundColor: dataColors[label],
            borderColor: dataColors[label],
            borderWidth: 1,
        };
    }) : [];

    const bar: ChartData<'bar'> = {
        labels: label,
        datasets: datasets,
    };

    return (
        <div>
            <h1>Window Chart</h1>
            <Bar data={bar} options={options} />
        </div>
    );
};

export default ChartComponent;
