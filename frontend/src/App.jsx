import { useState } from 'react';
import RegisterForm from './register/Form.jsx';
import LoginForm from './login/Form.jsx';
import MainPage from './MainPage/mainpage.jsx';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';

function App() {
    return (
        <Router>
            <Routes>
                <Route path='/' element={<MainPage />} />
                <Route path="/register" element={<RegisterForm />} />
                <Route path="/login" element={<LoginForm />} />
            </Routes>
        </Router>
    );
}

export default App;

