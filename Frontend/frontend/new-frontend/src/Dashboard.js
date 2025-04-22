import React, { useState } from 'react';
import './App.css';

function Dashboard() {
  const [inventory, setInventory] = useState([]);
  const [showInventory, setShowInventory] = useState(false);

  const handleLogout = () => {
    localStorage.removeItem('loggedIn');
    window.location.href = '/';
  };

  const fetchInventory = () => {
    fetch('http://localhost:8080/api/inventory')
      .then(res => res.json())
      .then(data => {
        setInventory(data);
        setShowInventory(true);
      })
      .catch(err => console.error("Error fetching inventory:", err));
  };

  const closeInventory = () => setShowInventory(false);

  return (
    <div className="App">
      <header className="App-header">
        <h1>QuickStock Dashboard</h1>
        <button onClick={handleLogout}>Logout</button>
        <p>Manage inventory, sales, and settings</p>
      </header>

      <main className="App-main">
        <section className="card">
          <h2>Inventory</h2>
          <button onClick={fetchInventory}>View Inventory</button>
          <button onClick={() => alert("Add new item...")}>Add Item</button>
        </section>

        <section className="card">
          <h2>Sales</h2>
          <button onClick={() => alert("View sales...")}>View Sales</button>
          <button onClick={() => alert("Record sale...")}>Record Sale</button>
        </section>

        <section className="card">
          <h2>Settings</h2>
          <button onClick={() => alert("Opening settings...")}>Open Settings</button>
        </section>
      </main>

      {showInventory && (
        <div className="modal">
          <div className="modal-content">
            <h3>Inventory List</h3>
            <ul>
            {inventory.slice(0, 10).map(item => (
                <li key={item.id}>
                  {item.name} — ${item.price} ({item.stock} in stock)
                </li>
              ))}
            </ul>
            <button onClick={closeInventory}>Close</button>
          </div>
        </div>
      )}

      <footer className="App-footer">
        <p>QuickStock Rust Edition &copy; 2025</p>
      </footer>
    </div>
  );
}

export default Dashboard;
