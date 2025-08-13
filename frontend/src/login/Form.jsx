import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import './LoginForm.css';
import Navbar from '../MainPage/Navbar';

function FormLogin({ setLoggedIn }) { // accept setLoggedIn as a prop
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const navigate = useNavigate();

    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const response = await fetch("http://localhost:42069/api/login", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ email, password }),
                credentials: 'include',
            });

            if (!response.ok) {
                throw new Error('Login failed');
            }

            const data = await response.json();
            console.log('Success: ', data);

            // Update parent state
            if (setLoggedIn) setLoggedIn(true);

            // Redirect to main page
            navigate('/');
        } catch (error) {
            console.error('Error: ', error);
            alert('Login failed!');
        }
    }

    return (
        <div>
            <Navbar/>
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
        </div>
    )
}
export default FormLogin;

