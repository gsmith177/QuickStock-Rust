import React, { useState, useEffect } from 'react';
import './Sales.css';
import { LineChart } from '@mui/x-charts/LineChart';
import dayjs from "dayjs";
import { PieChart } from '@mui/x-charts';

function Sales() {
  const [inventory, setInventory] = useState([]);

  useEffect(() => {
    fetchInventory();
    console.log(inventory);
  }, []);

  useEffect(() => {
    console.log("Inventory data:", inventory);
    console.log("Dates:", inventory.map(item => new Date(item.date_stocked)));
  }, [inventory]);

  const fetchInventory = () => {
    fetch("http://localhost:8080/products")
      .then(res => {
        if (!res.ok) {
          throw new Error("Network response was not ok");
        }
        return res.json();
      })
      .then(data => {
        setInventory(data);
      })
      .catch(err => {
        console.error("Error fetching inventory:", err);
        alert("Failed to load inventory.");
      });
  };

  const validDates = inventory.filter(item => {
    const date = new Date(item.date_stocked);
    return !isNaN(date);
  });

  return (
    <div className="sales-wrapper">
      <div className="sales-grid">
        <div className="sales-box">
          <label> Revenue Over Time </label>
          <LineChart
            xAxis={[
              {
                data: validDates.map(item => new Date(item.date_stocked)),
                scaleType: "time",
                valueFormatter: (date) => dayjs(date).format("MMM D")
              },
            ]}
            series={[
              {
                data: validDates.map(item => ((item.sell_price) * (item.quantity_sold))),
              },
            ]}
            height={300}
          />
        </div>
        <div className="sales-box">
          <label> Units Sold Over Time </label>
          <LineChart
            xAxis={[
              {
                data: validDates.map(item => new Date(item.date_stocked)),
                scaleType: "time",
                valueFormatter: (date) => dayjs(date).format("MMM D")
              }
            ]}
            series={[
              {
                data: validDates.map(item => (item.quantity_sold)),
              },
            ]}
            height={300}
          />

        </div>
        <div className="sales-box full-width">
          <label>Revenue Distribution by Product</label>
          <PieChart
            series={[
              {
                data: validDates.map(item => ({
                  id: item.id,
                  value: ((item.sell_price) * (item.quantity_sold)),
                  label: item.name
                }))
              }
            ]}
          />
        </div>
      </div>
      <button className="return-button" onClick={() => window.location.href = '/main'}>
        Return to Main
      </button>
    </div>
  );
}

export default Sales;
