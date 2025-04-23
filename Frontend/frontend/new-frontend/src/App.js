import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Login from './Login';
import Dashboard from './Dashboard';
import Settings from './Settings';

function App() {
  const [loggedIn, setLoggedIn] = useState(localStorage.getItem('loggedIn') === 'true');

  useEffect(() => {
    const handleStorageChange = () => {
      setLoggedIn(localStorage.getItem('loggedIn') === 'true');
    };

    window.addEventListener('storage', handleStorageChange);
    return () => window.removeEventListener('storage', handleStorageChange);
  }, []);

  return (
    <Router>
      <Routes>
        <Route path="/" element={loggedIn ? <Navigate to="/main" /> : <Login onLogin={() => setLoggedIn(true)} />} />
        <Route path="/main" element={loggedIn ? <Dashboard /> : <Navigate to="/" />} />
        <Route path="/settings" element={loggedIn ? <Settings /> : <Navigate to="/" />} />
      </Routes>
    </Router>
  );
}

export default App;
