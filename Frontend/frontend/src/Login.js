import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import './Login.css';

const Login = ({ onLoginSuccess, onLogout }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [securityAnswer, setSecurityAnswer] = useState('');
  const [showSecurity, setShowSecurity] = useState(false);
  const [userData, setUserData] = useState({});
  const navigate = useNavigate();

  /*
    COMMENTED OUT TO TEST FRONTEND
    */
  // useEffect(() => {
  //   fetchUserData();
  // }, []);

  // const fetchUserData = async () => {
  //   const data = await window.dataUtils.load_user_data();
  //   setUserData(data);
  // };

  function checkLogin() {
    navigate('/main');

    /*
    COMMENTED OUT TO TEST FRONTEND
    */
    // if (username === userData.username && password === userData.password) {
    //   alert('Welcome!');
    //   resetFields();
    //   onLoginSuccess();
    // } else {
    //   alert('Invalid username or password.');
    // }
  };

  function toggleSecurityQuestion() {
    if (!userData.hint) {
      alert('No security question has been made.');
      return;
    }
    setShowSecurity(!showSecurity);
  };

  function checkSecurityAnswer() {
    if (!userData.hint) {
      alert('No security question has been made.');
      return;
    }
    if (securityAnswer.toLowerCase() === userData.answer) {
      alert('Welcome! Please go to settings to update username/password.');
      resetFields();
      onLoginSuccess();
    } else {
      alert('Incorrect answer. Try again.');
      setSecurityAnswer('');
    }
  };

  function resetFields() {
    setUsername('');
    setPassword('');
    setShowPassword(false);
    setSecurityAnswer('');
    setShowSecurity(false);
  };

  return (
    <div className="login-container">
      <h2>Sign in</h2>
      <label>Username:</label>
      <input type="text" value={username} onChange={(e) => setUsername(e.target.value)} />

      <label>Password:</label>
      <input
        type={showPassword ? 'text' : 'password'}
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <label>
        <input
          type="checkbox"
          checked={showPassword}
          onChange={() => setShowPassword(!showPassword)}
        />{' '}
        Show Password
      </label>

      <button onClick={checkLogin}>Login</button>
      <button onClick={toggleSecurityQuestion}>Forgot Username or Password</button>

      {showSecurity && (
        <>
          <label>{userData.hint || 'No hint available'}</label>
          <input
            type="text"
            value={securityAnswer}
            onChange={(e) => setSecurityAnswer(e.target.value)}
          />
          <button onClick={checkSecurityAnswer}>Submit</button>
        </>
      )}
    </div>
  );
};

export default Login;