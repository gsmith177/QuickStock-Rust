import React, { useState, useEffect } from 'react';
import './App.css';

function Dashboard() {
  // Inventory data and UI state
  const [inventory, setInventory] = useState([]);
  const [showInventory, setShowInventory] = useState(false);
  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 10;

  // Add Item state
  const [showAddForm, setShowAddForm] = useState(false);
  const [newItem, setNewItem] = useState({ name: '', category: '', price: '', stock: '' });

  // Edit Item state
  const [editingItem, setEditingItem] = useState(null);
  const [editForm, setEditForm] = useState({ name: '', category: '', price: '', stock: '' });

  // Logout
  const handleLogout = () => {
    localStorage.removeItem('loggedIn');
    window.location.href = '/';
  };

  // Fetch inventory from backend
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
      setShowInventory(true);
    })
    .catch(err => {
      console.error("Error fetching inventory:", err);
      alert("Failed to load inventory.");
    });
  };
  // Delete an item
  const handleDelete = (id) => {
    if (!window.confirm("Delete this item?")) return;
    fetch(`http://localhost:8080/products/${id}`, { method: 'DELETE' })
    .then(() => fetchInventory());
  };

  // Begin editing an item
  const handleEdit = (item) => {
    setEditingItem(item);
    setEditForm({
      name: item.name,
      category: item.category,
      price: item.price,
      stock: item.stock
    });
  };

  // Save edited item
  const handleEditSave = () => {
    fetch(`http://localhost:8080/products/${editingItem.id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(editForm)
    })
      .then(() => {
        setEditingItem(null);
        fetchInventory();
      });
  };

  // Add a new item
  const handleAddItem = () => {
    fetch('http://localhost:8080/products', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(newItem)
    })
      .then(res => {
        if (!res.ok) throw new Error(`Failed to add item: ${res.statusText}`);
        setShowAddForm(false);
        setNewItem({ name: '', category: '', price: '', stock: '' });
        fetchInventory();
      })
      .catch(err => {
        console.error("Error adding item:", err);
        alert("Failed to add item.");
      });
  };
  

  // Close inventory modal
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
          <button onClick={fetchInventory}>Display Inventory</button>
          <button onClick={() => setShowAddForm(true)}>Add Item</button>
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

      {/* Inventory Modal */}
      {showInventory && (
        <div className="modal">
          <div className="modal-content">
            <h3>Inventory List</h3>
            <ul>
              {inventory
                .slice(currentPage * itemsPerPage, (currentPage + 1) * itemsPerPage)
                .map(item => (
                  <li key={item.id}>
                    {item.name} — {item.stock} (${item.price})
                    <button onClick={() => handleEdit(item)}>Edit</button>
                    <button onClick={() => handleDelete(item.id)}>Delete</button>
                  </li>
                ))
              }
            </ul>
            <div style={{ display: 'flex', justifyContent: 'space-between' }}>
              <button disabled={currentPage === 0} onClick={() => setCurrentPage(currentPage - 1)}>
                Previous
              </button>
              <button disabled={(currentPage + 1) * itemsPerPage >= inventory.length}
                      onClick={() => setCurrentPage(currentPage + 1)}>
                Next
              </button>
            </div>
            <button onClick={closeInventory}>Close</button>
          </div>
        </div>
      )}

      {/* Add Item Modal */}
      {showAddForm && (
        <div className="modal">
          <div className="modal-content">
            <h3>Add New Item</h3>
            <input
              placeholder="Name"
              value={newItem.name}
              onChange={e => setNewItem({ ...newItem, name: e.target.value })}
            /><br/>
            <input
              placeholder="Category"
              value={newItem.category}
              onChange={e => setNewItem({ ...newItem, category: e.target.value })}
            /><br/>
            <input
              placeholder="Price"
              type="number"
              value={newItem.price}
              onChange={e => setNewItem({ ...newItem, price: parseFloat(e.target.value) })}
            /><br/>
            <input
              placeholder="Stock"
              type="number"
              value={newItem.stock}
              onChange={e => setNewItem({ ...newItem, stock: parseInt(e.target.value) })}
            /><br/>
            <button onClick={handleAddItem}>Add</button>
            <button onClick={() => setShowAddForm(false)}>Cancel</button>
          </div>
        </div>
      )}

      {/* Edit Item Modal */}
      {editingItem && (
        <div className="modal">
          <div className="modal-content">
            <h3>Edit Item</h3>
            <input
              value={editForm.name}
              onChange={e => setEditForm({ ...editForm, name: e.target.value })}
            /><br/>
            <input
              value={editForm.category}
              onChange={e => setEditForm({ ...editForm, category: e.target.value })}
            /><br/>
            <input
              type="number"
              value={editForm.price}
              onChange={e => setEditForm({ ...editForm, price: parseFloat(e.target.value) })}
            /><br/>
            <input
              type="number"
              value={editForm.stock}
              onChange={e => setEditForm({ ...editForm, stock: parseInt(e.target.value) })}
            /><br/>
            <button onClick={handleEditSave}>Save</button>
            <button onClick={() => setEditingItem(null)}>Cancel</button>
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
