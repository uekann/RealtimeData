"use client"; // クライアントコンポーネントとして指定

import React from 'react';
import "./table.css";

const TableComponent = ({ records }) => {
    const keys = records.length > 0 ? records.map((record) => record["timestamp"]) : [];
    console.log(keys);
    return (
        <div>
            <h1>Window Table</h1>
            <table>
                <thead>
                    <tr>
                        <th>Stock</th>
                        <th>Open</th>
                        <th>Low</th>
                        <th>High</th>
                        <th>Close</th>
                    </tr>
                </thead>
                <tbody>
                    {records.map((record) => (
                        <tr key={record["data"]["timestamp"]}>
                            <td>{record["data"]["stock"]}</td>
                            <td>{record["data"]["open"]}</td>
                            <td>{record["data"]["low"]}</td>
                            <td>{record["data"]["high"]}</td>
                            <td>{record["data"]["close"]}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
}

export default TableComponent;