import { useEffect, useState } from 'react';
import './mainpage.css';
import Navbar from './Navbar.jsx';
import PageLogedIn from './LoggedIn.jsx';
import PageLogedOut from './NotLoggedIn.jsx';

function MainPage() {
    const [loggedIn, setLoggedIn] = useState(false);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        // Check login status on mount
        const checkLogin = async () => {
            try {
                const response = await fetch('http://localhost:42069/api/profile', {
                    method: 'GET',
                    credentials: 'include', // important to send cookies
                });
                setLoggedIn(response.ok);
            } catch (error) {
                console.error('Error checking session:', error);
                setLoggedIn(false);
            } finally {
                setLoading(false);
            }
        };

        checkLogin();
    }, []);

    if (loading) return <div>Loading...</div>;

    return (
        <div>
            {/* Pass state and setter to Navbar */}
            <Navbar loggedIn={loggedIn} setLoggedIn={setLoggedIn} />
            <div className="title">
                {loggedIn ? <PageLogedIn /> : <PageLogedOut />}
            </div>
        </div>
    );
}

export default MainPage;
