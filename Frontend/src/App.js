import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Login from './Login';
import Dashboard from './Dashboard';
import Settings from './Settings';
import Sales from './Sales';

function App() {
  const [loggedIn, setLoggedIn] = useState(localStorage.getItem('loggedIn') === 'true');

  // Handle login state and changes to the login
  useEffect(() => {
    const handleStorageChange = () => {
      setLoggedIn(localStorage.getItem('loggedIn') === 'true');
    };

    window.addEventListener('storage', handleStorageChange);

    // Clean event listener
    return () => window.removeEventListener('storage', handleStorageChange);
  }, []);

  return (
    <Router>
      <Routes>
        {/* Prevent access to other pages unless user has logged in*/}
        <Route path="/" element={loggedIn ? <Navigate to="/main" /> : <Login onLogin={() => setLoggedIn(true)} />} />
        <Route path="/main" element={loggedIn ? <Dashboard /> : <Navigate to="/" />} />
        <Route path="/settings" element={loggedIn ? <Settings /> : <Navigate to="/" />} />
        <Route path="/sales" element={loggedIn ? <Sales /> : <Navigate to="/" />} />
      </Routes>
    </Router>
  );
}

export default App;
