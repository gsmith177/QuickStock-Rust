import React from 'react';
import { useNavigate } from 'react-router-dom';
import './Main.css';

function Main() {
  const navigate = useNavigate();

  return (
    <div className="main-wrapper">
      <div className="main-container">
        <h1>Main Menu</h1>
        <button onClick={() => navigate('/sales')}>Sales</button>
        <button onClick={() => navigate('/inventory')}>Inventory</button>
        <button onClick={() => navigate('/settings')}>Settings</button>
      </div>
    </div>
  );
}

export default Main;
