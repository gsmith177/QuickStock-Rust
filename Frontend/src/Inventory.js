import React from 'react';
import { useNavigate } from 'react-router-dom';
import './App.css';

function Inventory() {
  const navigate = useNavigate();

  // Since the inventory mainly consists of pop ups, there are minimal additions
  // needed to this component

  return (
    <div>
      <h2>Inventory Page</h2>
      <button className="return-button" onClick={() => navigate('/main')}>
        Return to Main
      </button>
    </div>
  );
}

export default Inventory;
