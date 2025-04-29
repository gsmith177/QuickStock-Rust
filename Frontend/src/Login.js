// Frontend/src/Login.js
import './Login.css';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

function Login({ onLogin }) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleLogin = async (e) => {
    e.preventDefault();
    setError('');

    try {
      const res = await fetch('http://localhost:8080/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',   // in case you later expand to cookies/sessions
        body: JSON.stringify({ username, password }),
      });

      if (res.ok) {
        // backend returned 200 = credentials match
        localStorage.setItem('loggedIn', 'true');
        onLogin();               
        navigate('/main');       // note: your App.js expects "/main"
      } else {
        // 401 or other status
        setError('Invalid username or password');
      }
    } catch (err) {
      console.error('Login error:', err);
      setError('Server error—please try again');
    }
  };

  return (
    <div className="login-container">
      <h2>Log In</h2>
      <form onSubmit={handleLogin}>
        <label>
          Username
          <input
            type="text"
            value={username}
            onChange={e => setUsername(e.target.value)}
            required
          />
        </label>

        <label>
          Password
          <input
            type={showPassword ? 'text' : 'password'}
            value={password}
            onChange={e => setPassword(e.target.value)}
            required
          />
        </label>

        <label className="show-password">
          <input
            type="checkbox"
            checked={showPassword}
            onChange={() => setShowPassword(!showPassword)}
          />{' '}
          Show Password
        </label>

        <button type="submit">Login</button>
      </form>

      {error && <p className="login-error">{error}</p>}
    </div>
  );
}

export default Login;
