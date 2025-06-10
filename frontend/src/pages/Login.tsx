import React, { useState } from 'react';
import '../App.css'; // Assuming you want to reuse some global styles or will add custom styles here

const BASE_URL = "http://localhost:3000"; // 后端 API 地址

const LoginPage: React.FC = () => {
  const [isLogin, setIsLogin] = useState(true);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [message, setMessage] = useState(''); // 用于显示消息

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setMessage(''); // 清除之前的消息

    const endpoint = isLogin ? "/login" : "/register"; // 登录使用 /login，注册使用 /register
    const method = "POST";
    const body = isLogin ? { email, password } : { email, password };

    // 注册时检查密码是否一致
    if (!isLogin && password !== confirmPassword) {
      setMessage("Passwords do not match!");
      return;
    }

    try {
      const response = await fetch(`${BASE_URL}${endpoint}`, {
        method: method,
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(body),
      });

      const data = await response.json();

      if (response.ok) {
        setMessage(data.message || (isLogin ? "Login successful!" : "Registration successful!"));
        // 根据需要，登录成功后可以跳转到其他页面
        // 例如：history.push('/dashboard');
      } else {
        setMessage(data.message || (isLogin ? "Login failed!" : "Registration failed!"));
      }
    } catch (error) {
      console.error('Error during API call:', error);
      setMessage("Network error or server unreachable.");
    }
    // 清空密码字段
    setPassword('');
    setConfirmPassword('');
  };

  return (
    <div className="auth-container">
      <div className="auth-card">
        <h2>{isLogin ? 'Login' : 'Register'}</h2>
        <form onSubmit={handleSubmit}>
          <div className="input-group">
            <label htmlFor="email">Email</label>
            <input
              type="email"
              id="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>
          <div className="input-group">
            <label htmlFor="password">Password</label>
            <input
              type="password"
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          </div>
          {!isLogin && (
            <div className="input-group">
              <label htmlFor="confirmPassword">Confirm Password</label>
              <input
                type="password"
                id="confirmPassword"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                required
              />
            </div>
          )}
          <button type="submit" className="auth-button">
            {isLogin ? 'Login' : 'Register'}
          </button>
        </form>
        {message && <p className="auth-message">{message}</p>}
        <p className="toggle-link">
          {isLogin ? 'Don\'t have an account?' : 'Already have an account?'}{' '}
          <span onClick={() => {
            setIsLogin(!isLogin);
            setMessage(''); // 切换时清除消息
            setEmail('');
            setPassword('');
            setConfirmPassword('');
          }}>
            {isLogin ? 'Register' : 'Login'}
          </span>
        </p>
      </div>
    </div>
  );
};

export default LoginPage; 