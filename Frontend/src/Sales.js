import React, { useState, useEffect } from 'react';
import './Sales.css';
import { LineChart } from '@mui/x-charts/LineChart';
import dayjs from "dayjs";
import { PieChart, BarChart} from '@mui/x-charts';

function Sales() {
  const [inventory, setInventory] = useState([]);

  useEffect(() => {
    fetchInventory();
  }, []);

  const fetchInventory = () => {
    fetch("http://localhost:8080/products")
      .then(res => {
        if (!res.ok) {
          throw new Error("Network response was not ok");
        }
        return res.json();
      })
      .then(data => {
        uniqueInventoryItems(data);
      })
      .catch(err => {
        console.error("Error fetching inventory:", err);
        alert("Failed to load inventory.");
      });
  };

  const uniqueInventoryItems = (items) => {

    let uniqueNames = [];
    let uniqueObjects = [];

    for (let i = 0; i < items.length; i++) {

      const item = items[i];
      let index = uniqueNames.indexOf(item.name);

      if (index !== -1) {
        uniqueObjects[index].quantity_sold.push(item.quantity_sold);
        uniqueObjects[index].date_stocked.push(new Date(item.date_stocked));
      }
      else {
        uniqueNames.push(item.name);
        uniqueObjects.push({...item, quantity_sold: [item.quantity_sold],
          date_stocked: [new Date(item.date_stocked)],
        });
      }
    }

    setInventory(uniqueObjects);
  }

  const salesOverTime = inventory.flatMap(item =>
    item.date_stocked.map((date, index) => ({
      date,
      quantity: item.quantity_sold[index]
    }))
  )

  if (inventory.length === 0) return <div>Loading...</div>;

  return (
    <div className="sales-wrapper">
      <div className="sales-grid">
        <div className="sales-box">
          <label>Revenue</label>
          <BarChart
            dataset={inventory}
            series={[
              {
                data: inventory.map(item => item.sell_price * (item.quantity_sold.reduce((a, b) => a + b, 0))),
                label: "Total Sales",
              },
              {
                data: inventory.map(item => (item.cost_price)),
                label: "Cost Price",
              },
              {
                data: inventory.map(item => (item.sell_price)),
                label: "Sell Price",
              }
            ]}
            xAxis={[{ scaleType: 'band', dataKey: 'name' }]}
          />
        </div>
        <div className="sales-box">
          <label> Units Sold Over Time </label>
          <LineChart
            xAxis={[
              {
                data: salesOverTime.map(point => point.date),
                scaleType: "time",
                valueFormatter: (date) => dayjs(date).format("MMM D")
              }
            ]}
            series={[
              {
                data: salesOverTime.map(point => point.quantity),
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
                data: inventory.map(item => ({
                  id: item.id,
                  value: item.sell_price * item.quantity_sold.reduce((a, b) => a + b, 0),
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
