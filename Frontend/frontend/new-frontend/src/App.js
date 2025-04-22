import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Login from './Login';
import Main from './Main'; // This will be your new `main` page

function App() {
  const loggedIn = localStorage.getItem('loggedIn') === 'true';

  return (
    <Router>
      <Routes>
        <Route path="/" element={loggedIn ? <Navigate to="/main" /> : <Login />} />
        <Route path="/main" element={loggedIn ? <Main /> : <Navigate to="/" />} />
      </Routes>
    </Router>
  );
}

export default App;
