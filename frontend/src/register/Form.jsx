import { useState } from 'react';
import { Link } from 'react-router-dom';
import './RegisterForm.css';
function FormRegister() {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const response = await fetch("http://localhost:42069/api/register", {
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
            alert('Registration succesful!');
        } catch (error) {
            console.error('Error: ', error);
            alert('Registration failed!');
        }
    }
    return (
        <div className='center'>
            <form onSubmit={handleSubmit}>
                <h2 className="h2_title"> Please register to out site </h2>
                <input type="text" id="input" name="email" placeholder="Enter your email" onChange={(e) => setEmail(e.target.value)} required />
                <input type="password" id="input" name="password" placeholder="Enter your password" onChange={(e) => setPassword(e.target.value)} required />
                <div className="login_link">
                    I already have an account <Link to="/login">Login</Link>
                </div>
                <button className="submit_button"> Submit </button>
            </form>
        </div>
    )
}
export default FormRegister
