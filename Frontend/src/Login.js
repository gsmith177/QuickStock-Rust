import './Login.css';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

function Login({ onLogin }) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [error, setError] = useState('');
  const navigate = useNavigate();

  // Handle logic for logins
  const handleLogin = async (e) => {
    e.preventDefault();
    setError('');
    try {
      // API call to the backend for login credentials
      const res = await fetch('http://localhost:8080/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ username, password }),
      });

      // Handle valid logins
      if (res.ok) {
        localStorage.setItem('loggedIn', 'true');
        localStorage.setItem('username', username);
        onLogin();
        navigate('/main');
      } else {
        // Handle invalid logins
        setError('Invalid username or password');
      }
    } catch {
      setError('Server error—please try again');
    }
  };

  return (
    // Create outer login container
    <div className="login-container">
      <h2>Log In</h2>
      <form onSubmit={handleLogin}>
        <label>
          Username
          {/* Handle changes to the username field */}
          <input
            type="text"
            value={username}
            onChange={e => setUsername(e.target.value)}
            required
          />
        </label>
        <label>
          Password
          {/* Handle changes to the password field */}
          <input
            type={showPassword ? 'text' : 'password'}
            value={password}
            onChange={e => setPassword(e.target.value)}
            required
          />
        </label>
        {/* Toggle password visibility */}
        <label className="show-password">
          <input
            type="checkbox"
            checked={showPassword}
            onChange={() => setShowPassword(!showPassword)}
          /> Show Password
        </label>
        <button type="submit">Login</button>
      </form>
      {error && <p className="login-error">{error}</p>}
    </div>
  );
}

export default Login;
