import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import './Navbar.css';

const Navbar = () => {
  const { user, logout, isAdmin, isAuthenticated } = useAuth();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <nav className="navbar">
      <div className="navbar-brand">
        <Link to="/">Boxing Gym</Link>
      </div>

      <div className="navbar-menu">
        <Link to="/" className="nav-link">Home</Link>
        <Link to="/courses" className="nav-link">Courses</Link>
        <Link to="/schedules" className="nav-link">Schedule</Link>
        <Link to="/blog" className="nav-link">Blog</Link>

        {isAuthenticated ? (
          <>
            <Link to="/my-bookings" className="nav-link">My Bookings</Link>
            {isAdmin() && (
              <div className="nav-dropdown">
                <span className="nav-link dropdown-trigger">Admin</span>
                <div className="dropdown-content">
                  <Link to="/admin/members">Members</Link>
                  <Link to="/admin/courses">Courses</Link>
                  <Link to="/admin/schedules">Schedules</Link>
                  <Link to="/admin/announcements">Announcements</Link>
                  <Link to="/admin/blog">Blog Posts</Link>
                  <Link to="/admin/plans">Membership Plans</Link>
                </div>
              </div>
            )}
            <div className="nav-user">
              <span className="user-name">{user?.first_name}</span>
              <span className="user-role">({user?.role})</span>
              <button onClick={handleLogout} className="btn-logout">Logout</button>
            </div>
          </>
        ) : (
          <div className="nav-auth">
            <Link to="/login" className="nav-link">Login</Link>
            <Link to="/register" className="btn-register">Register</Link>
          </div>
        )}
      </div>
    </nav>
  );
};

export default Navbar;
