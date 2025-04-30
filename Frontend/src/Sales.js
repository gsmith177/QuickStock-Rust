import React, { useState, useEffect } from 'react';
import './Sales.css';
import { LineChart } from '@mui/x-charts/LineChart';
import dayjs from "dayjs";
import { PieChart } from '@mui/x-charts';

function Sales() {
  // Handle the inventory state and the arrays for the unique objects
  const [inventory, setInventory] = useState([]);
  const [salesPerItem, setSalesPerItem] = useState([]);
  const [profitPerItem, setProfitPerItem] = useState([]);

  // On page load, call fetchInventory
  useEffect(() => {
    fetchInventory();
  }, []);

  // Fetches the inventory from the backend
  const fetchInventory = () => {
    fetch("http://localhost:8080/products")
      .then(res => {
        if (!res.ok) {
          throw new Error("Network response was not ok");
        }
        return res.json();
      })
      .then(data => {
        // Call the functions to find unique items
        uniqueInventoryItems(data);
      })
      .catch(err => {
        console.error("Error fetching inventory:", err);
        alert("Failed to load inventory.");
      });
  };

  // Goes through the inventory and removes repeated items
  const uniqueInventoryItems = (items) => {

    let uniqueNames = [];
    let uniqueObjects = [];

    for (let i = 0; i < items.length; i++) {

      const item = items[i];
      let index = uniqueNames.indexOf(item.name);

      // Check if the item has already been added to the unique items list
      if (index !== -1) {
        uniqueObjects[index].quantity_sold.push(item.quantity_sold);
        uniqueObjects[index].date_stocked.push(new Date(item.date_stocked));
      }
      // If the item is not in the unique items list, add it with quantity_sold and date_stocked as arrays
      else {
        uniqueNames.push(item.name);
        uniqueObjects.push({
          ...item, quantity_sold: [item.quantity_sold],
          date_stocked: [new Date(item.date_stocked)],
        });
      }
    }
    setInventory(uniqueObjects);

    // Add all repeated items to their appropriate corresponding arrays for the quantity-date line graph
    let temp1 = [];
    uniqueObjects.forEach(item => {
      temp1[item.name] = item.date_stocked.map((date, index) =>
      ({
        date,
        quantity: item.quantity_sold[index]
      }))
    });
    setSalesPerItem(temp1);

    // Add all repeated items to their appropriate corresponding arrays for the sales-date line graph
    let temp2 = {};
    uniqueObjects.forEach(item => {
      temp2[item.name] = item.date_stocked.map((date, index) =>
      ({
        date,
        quantity: item.quantity_sold[index],
        sell_price: item.sell_price
      }));
    });
    setProfitPerItem(temp2);
  }

  // The following two functions were created using ChatGPT's assistance 
  // Get all the dates from the item's array and sort them
  const getDates = (data) => {
    const dates = new Set();
    Object.values(data).forEach(points => {
      points.forEach(point => dates.add(point.date.toISOString()));
    });
    return Array.from(dates)
      .map(dateStr => new Date(dateStr))
      .sort((a, b) => a - b);
  };

  // Map all the points for the quantity-date line graph
  const quantitiesWithDates = (points, allDates) => {
    const quantitiesMapped = {};
    points.forEach(dailyItem => {
      quantitiesMapped[dailyItem.date.toISOString()] = dailyItem.quantity;
    });
    return allDates.map(date => quantitiesMapped[date.toISOString()] ?? 0);
  };

  // Map all the points for the sales-date line graph
  const salesWithDates = (points, allDates) => {
    const salesMapped = {};
    points.forEach(dailyItem => {
      salesMapped[dailyItem.date.toISOString()] = (dailyItem.quantity * dailyItem.sell_price);
    });
    return allDates.map(date => salesMapped[date.toISOString()] ?? 0);
  };

  // Maintain a loading screen when the inventory cannot be loaded
  if (inventory.length === 0) return <div>Loading...</div>;

  return (
    <div className="sales-wrapper">
      <div className="sales-grid">
        {/* Box for the quantity-date line graph */}
        <div className="sales-box">
          <label>Revenue</label>
          <LineChart
            // Plot the quantity sold (y-axis) against the dates (x-axis)
            xAxis={[
              {
                data: getDates(profitPerItem),
                scaleType: "time",
                valueFormatter: (date) => dayjs(date).format("MMM D")
              }
            ]}
            series={Object.keys(profitPerItem).map((name) => ({
              label: name,
              data: salesWithDates(profitPerItem[name], getDates(profitPerItem)),
            }))}
            height={300}
          />
        </div>
        {/* Box for the sales-date line graph */}
        <div className="sales-box">
          <label> Units Sold Over Time </label>
          <LineChart
            // Plot the profit (y-axis) against the dates (x-axis)
            xAxis={[
              {
                data: getDates(salesPerItem),
                scaleType: "time",
                valueFormatter: (date) => dayjs(date).format("MMM D")
              }
            ]}
            series={Object.keys(salesPerItem).map((name) => ({
              label: name,
              data: quantitiesWithDates(salesPerItem[name], getDates(salesPerItem)),
            }))}
            height={300}
          />
        </div>
        {/* Box for the pie chart comparing item sales*/}
        <div className="sales-box full-width">
          <label>Revenue Distribution by Product</label>
          <PieChart
            series={[
              {
                // Pie chart includes the profit for each item
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
        Return
      </button>
    </div>
  );
}

export default Sales;
