import React from 'react';
import './Sales.css';

function Sales() {
  return (
    <div className="sales-wrapper">
      <div className="sales-grid">
        <div className="sales-box">Chart 1</div>
        <div className="sales-box">Chart 2</div>
        <div className="sales-box full-width">Chart 3</div>
      </div>
      <button className="return-button" onClick={() => window.location.href = '/main'}>
        Return to Main
      </button>
    </div>
  );
}

export default Sales;
