import React from 'react';
import './App.css';

function Dashboard() {
  const handleLogout = () => {
    localStorage.removeItem('loggedIn');
    window.location.href = '/';
  };

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
          <button onClick={() => alert("Viewing inventory...")}>View Inventory</button>
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

      <footer className="App-footer">
        <p>QuickStock Rust Edition &copy; 2025</p>
      </footer>
    </div>
  );
}

export default Dashboard;
