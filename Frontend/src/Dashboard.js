import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './Dashboard.css';

function Dashboard() {
  // Inventory data and UI state
  const [inventory, setInventory] = useState([]);
  const [showInventory, setShowInventory] = useState(false);
  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 10;

  // Add Item state
  const [showAddForm, setShowAddForm] = useState(false);
  const [newItem, setNewItem] = useState({
    name: '',
    category: '',
    quantity: '',
    cost_price: '',
    sell_price: '',
    available: true,
    date_stocked: '',
    contact: '',
    quantity_sold: ''
  });
  // Edit Item state
  const [editingItem, setEditingItem] = useState(null);
  const [editForm, setEditForm] = useState({
    name: '',
    category: '',
    quantity: '',
    cost_price: '',
    sell_price: '',
    available: true,
    date_stocked: '',
    contact: '',
    quantity_sold: ''
  });
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
      quantity: item.quantity,
      cost_price: item.cost_price,
      sell_price: item.sell_price,
      available: item.available,
      date_stocked: item.date_stocked,
      contact: item.contact,
      quantity_sold: item.quantity_sold
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
        setNewItem({
          name: '',
          category: '',
          quantity: '',
          cost_price: '',
          sell_price: '',
          available: true,
          date_stocked: '',
          contact: '',
          quantity_sold: ''
        });
        fetchInventory();
      })
      .catch(err => {
        console.error("Error adding item:", err);
        alert("Failed to add item.");
      });
  };


  // Close inventory modal
  const closeInventory = () => setShowInventory(false);
  const navigate = useNavigate();

  return (
    <div className="Dashboard">
      <header className="Dashboard-header">
        <h1>QuickStock</h1>
        <p>Manage inventory, sales, and settings</p>
      </header>

      <main className="Dashboard-main">
        <section className="card">
          <h2>Inventory</h2>
          <button onClick={fetchInventory}>Display Inventory</button>
          <button onClick={() => setShowAddForm(true)}>Add Item</button>
        </section>

        <section className="card">
          <h2>Sales</h2>
          <button onClick={() => navigate('/sales')}>Sales</button>
        </section>

        <section className="card">
          <h2>Settings</h2>
          <button onClick={() => navigate('/settings')}>Settings</button>
        </section>
      </main>

      {/* Inventory Display */}
      {showInventory && (
        <div className="modal">
          <div className="modal-content">
            <h3>Inventory List</h3>
            {/* Table for displaying inventory items and their properties */}
            <table className="inventory-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Quantity</th>
                  <th>Price</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {inventory
                  .slice(currentPage * itemsPerPage, (currentPage + 1) * itemsPerPage)
                  .map(item => (
                    <tr key={item.id}>
                      <td>{item.name}</td>
                      <td>{item.quantity}</td>
                      <td>${item.sell_price}</td>
                      <td>
                        <button onClick={() => handleEdit(item)}>Edit</button>
                        <button onClick={() => handleDelete(item.id)}>Delete</button>
                      </td>
                    </tr>
                  ))}
              </tbody>
            </table>
            <br></br>
            <div style={{ display: 'flex', justifyContent: 'space-between' }}>
              <button disabled={currentPage === 0} onClick={() => setCurrentPage(currentPage - 1)}>
                Previous
              </button>
              <button disabled={(currentPage + 1) * itemsPerPage >= inventory.length}
                onClick={() => setCurrentPage(currentPage + 1)}>
                Next
              </button>
            </div>
            <br></br>
            <button onClick={closeInventory}>Close</button>
          </div>
        </div>
      )}

      {/* Add Item */}
      {showAddForm && (
        <div className="modal">
          <div className="modal-content">
            <h3>Add New Item</h3>
            <input
              placeholder="Name"
              value={newItem.name}
              onChange={e => setNewItem({ ...newItem, name: e.target.value })}
            /><br />
            <input
              placeholder="Category"
              value={newItem.category}
              onChange={e => setNewItem({ ...newItem, category: e.target.value })}
            /><br />
            <input
              placeholder="Quantity"
              type="number"
              value={newItem.quantity}
              onChange={e => setNewItem({ ...newItem, quantity: parseInt(e.target.value) || 0 })}
            /><br />
            <input
              placeholder="Cost Price"
              type="number"
              value={newItem.cost_price}
              onChange={e => setNewItem({ ...newItem, cost_price: parseFloat(e.target.value) || 0 })}
            /><br />
            <input
              placeholder="Sell Price"
              type="number"
              value={newItem.sell_price}
              onChange={e => setNewItem({ ...newItem, sell_price: parseFloat(e.target.value) || 0 })}
            /><br />
            <label class="available-label">
              Available:
              <input
                type="checkbox"
                checked={newItem.available}
                onChange={e => setNewItem({ ...newItem, available: e.target.checked })}
              />
            </label><br />
            <input
              placeholder="Date Stocked (YYYY-MM-DD)"
              value={newItem.date_stocked}
              onChange={e => setNewItem({ ...newItem, date_stocked: e.target.value })}
            /><br />
            <input
              placeholder="Contact"
              value={newItem.contact}
              onChange={e => setNewItem({ ...newItem, contact: e.target.value })}
            /><br />
            <input
              placeholder="Quantity Sold"
              type="number"
              value={newItem.quantity_sold}
              onChange={e => setNewItem({ ...newItem, quantity_sold: parseInt(e.target.value) || 0 })}
            /><br />
            <button onClick={handleAddItem}>Add</button>
            <button onClick={() => setShowAddForm(false)}>Cancel</button>
          </div>
        </div>
      )}

      {/* Edit Item */}
      {editingItem && (
        <div className="modal">
          <div className="modal-content">
            <h3>Edit Item</h3>
            <input
              value={editForm.name}
              onChange={e => setEditForm({ ...editForm, name: e.target.value })}
            /><br />
            <input
              value={editForm.category}
              onChange={e => setEditForm({ ...editForm, category: e.target.value })}
            /><br />
            <input
              type="number"
              value={editForm.quantity}
              onChange={e => setEditForm({ ...editForm, quantity: parseInt(e.target.value) || 0 })}
            /><br />
            <input
              type="number"
              value={editForm.cost_price}
              onChange={e => setEditForm({ ...editForm, cost_price: parseFloat(e.target.value) || 0 })}
            /><br />
            <input
              type="number"
              value={editForm.sell_price}
              onChange={e => setEditForm({ ...editForm, sell_price: parseFloat(e.target.value) || 0 })}
            /><br />
            <label>
              Available:
              <input
                type="checkbox"
                checked={editForm.available}
                onChange={e => setEditForm({ ...editForm, available: e.target.checked })}
              />
            </label><br />
            <input
              value={editForm.date_stocked}
              onChange={e => setEditForm({ ...editForm, date_stocked: e.target.value })}
            /><br />
            <input
              value={editForm.contact}
              onChange={e => setEditForm({ ...editForm, contact: e.target.value })}
            /><br />
            <input
              type="number"
              value={editForm.quantity_sold}
              onChange={e => setEditForm({ ...editForm, quantity_sold: parseInt(e.target.value) || 0 })}
            /><br />
            <button onClick={handleEditSave}>Save</button>
            <button onClick={() => setEditingItem(null)}>Cancel</button>
          </div>
        </div>
      )}

      {/* Handle Logouts */}
      <div class="logout-container">
        <button class="logout-btn" onClick={handleLogout}>Logout</button>
      </div>

      <footer className="App-footer">
        <p>QuickStock Rust Edition &copy; 2025</p>
      </footer>
    </div>
  );
}

export default Dashboard;
