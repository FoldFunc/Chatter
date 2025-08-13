import { useState } from 'react';
import { Link } from 'react-router-dom';
import './LoginForm.css';
function FormLogin() {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const response = await fetch("http://localhost:42069/api/login", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ email, password }),
            });
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const data = await response.json();
            console.log('Succes: ', data);
            alert('Login succesful succesful!');
        } catch (error) {
            console.error('Error: ', error);
            alert('Login failed!');
        }
    }
    return (
        <div className='center'>
        <form onSubmit={handleSubmit}>
            <h2 className="h2_title">Please login with your email and password</h2>
            <input type="text" id="input" name="email" placeholder="Enter your email" onChange={(e) => setEmail(e.target.value)} required />
            <input type="password" id="input" name="password" placeholder="Enter your password" onChange={(e) => setPassword(e.target.value)} required />
            <div className="login_link">
                I do not have an account  <Link to="/register">Register</Link>
            </div>
            <button className="submit_button"> Submit </button>
        </form>
        </div>
    )
}
export default FormLogin
