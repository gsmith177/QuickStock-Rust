import React from 'react';
import { useNavigate } from 'react-router-dom';
import './App.css';


function Sales() {
  const navigate = useNavigate();

  return (
    <div>
      <h2>Sales Page</h2>
      <button className="return-button" onClick={() => navigate('/main')}>
        Return to Main
      </button>
    </div>
  );
}

export default Sales;
