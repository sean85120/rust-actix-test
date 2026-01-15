import { Outlet } from 'react-router-dom';
import Navbar from './Navbar';
import './Layout.css';

const Layout = () => {
  return (
    <div className="layout">
      <Navbar />
      <main className="main-content">
        <Outlet />
      </main>
      <footer className="footer">
        <p>&copy; 2024 Boxing Gym. All rights reserved.</p>
      </footer>
    </div>
  );
};

export default Layout;
