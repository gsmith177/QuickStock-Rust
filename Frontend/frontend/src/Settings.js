import React, { useState } from 'react';
import './App.css';

function Settings() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [message, setMessage] = useState('');

  const handleUpdate = async () => {
    try {
      const res = await fetch('http://127.0.0.1:3000/update-credentials', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password }),
      });

      const text = await res.text();
      if (res.ok) {
        setMessage('✅ Credentials updated successfully!');
      } else {
        setMessage(`❌ Error: ${text}`);
      }
    } catch (err) {
      setMessage(`❌ Failed to connect: ${err.message}`);
    }
  };

  return (
    <div>
      <h2>Update Login Credentials</h2>
      <input
        type="text"
        placeholder="New Username"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      /><br/>
      <input
        type="password"
        placeholder="New Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      /><br/>
      <button onClick={handleUpdate}>Update</button>
      <p>{message}</p>

      <button className="return-button" onClick={() => window.location.href = '/main'}>
        Return to Main
      </button>
    </div>
  );
}

export default Settings;
