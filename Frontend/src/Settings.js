import './Settings.css';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

export default function Settings() {
  const navigate = useNavigate();
  const storedUsername = localStorage.getItem('username') || '';
  const [oldUsername, setOldUsername] = useState(storedUsername);
  const [newUsername, setNewUsername] = useState(storedUsername);
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');

  const handleUpdate = async (e) => {
    e.preventDefault();
    setError('');
    setMessage('');

    if (newPassword !== confirmPassword) {
      setError('Passwords do not match');
      return;
    }

    try {
      const res = await fetch('http://localhost:8080/update_user', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({
          old_username: oldUsername,
          new_username: newUsername,
          new_password: newPassword,
        }),
      });

      if (res.ok) {
        localStorage.setItem('username', newUsername);
        setMessage('Credentials updated successfully');
      } else {
        const text = await res.text();
        setError(text || 'Update failed');
      }
    } catch {
      setError('Server error—please try again');
    }
  };

  const handleLogout = () => {
    localStorage.removeItem('loggedIn');
    localStorage.removeItem('username');
    navigate('/');
  };

  return (
    <div className="settings-container">
      <h2>Account Settings</h2>
      <form onSubmit={handleUpdate}>
        <label>
          Old Username
          <input
            type="text"
            value={oldUsername}
            onChange={(e) => setOldUsername(e.target.value)}
            required
          />
        </label>

        <label>
          New Username
          <input
            type="text"
            value={newUsername}
            onChange={(e) => setNewUsername(e.target.value)}
            required
          />
        </label>

        <label>
          New Password
          <input
            type="password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            required
          />
        </label>

        <label>
          Confirm Password
          <input
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            required
          />
        </label>

        <button type="submit">Update</button>
      </form>

      {message && <p className="success">{message}</p>}
      {error && <p className="error">{error}</p>}

      <button className="return-button" onClick={handleLogout}>
        Return
      </button>

    </div>
  );
}
