import { Link, useNavigate } from 'react-router-dom';
import './Navbar.css';

function Navbar({ loggedIn, setLoggedIn }) {
    const navigate = useNavigate();

    const handleLogout = async () => {
        try {
            const response = await fetch('http://localhost:42069/api/logout', {
                method: 'GET',
                credentials: 'include',
            });

            if (!response.ok) throw new Error('Logout failed');

            // Update parent state
            setLoggedIn(false);

            navigate('/');
        } catch (error) {
            console.error('Logout error:', error);
            alert('Failed to logout!');
        }
    };

    return (
        <nav className="navbar">
            <div className="nav_logo">MySite</div>
            <ul className="nav_links">
                <li><Link to="/">Home</Link></li>
                {!loggedIn && <li><Link to="/register">Register</Link></li>}
                {!loggedIn && <li><Link to="/login">Login</Link></li>}
                {loggedIn && (
                    <li>
                        <button className="logout_button" onClick={handleLogout}>
                            Logout
                        </button>
                    </li>
                )}
            </ul>
        </nav>
    );
}

export default Navbar;
