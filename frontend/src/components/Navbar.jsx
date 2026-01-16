import { Link, useNavigate, useLocation } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import './Navbar.css';

const Navbar = () => {
  const { user, logout, isAdmin, isAuthenticated } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  const isActive = (path) => location.pathname === path;

  return (
    <nav className="navbar">
      <div className="navbar-brand">
        <Link to="/">
          <span className="brand-icon">🥊</span>
          <span>Knockout</span>
        </Link>
      </div>

      <div className="navbar-menu">
        <Link to="/" className={`nav-link ${isActive('/') ? 'active' : ''}`}>
          Home
        </Link>
        <Link to="/courses" className={`nav-link ${isActive('/courses') ? 'active' : ''}`}>
          Courses
        </Link>
        <Link to="/schedules" className={`nav-link ${isActive('/schedules') ? 'active' : ''}`}>
          Schedule
        </Link>
        <Link to="/blog" className={`nav-link ${isActive('/blog') ? 'active' : ''}`}>
          Blog
        </Link>

        {isAuthenticated ? (
          <>
            <Link to="/my-bookings" className={`nav-link ${isActive('/my-bookings') ? 'active' : ''}`}>
              My Bookings
            </Link>
            {isAdmin() && (
              <div className="nav-dropdown">
                <span className="nav-link dropdown-trigger">Admin</span>
                <div className="dropdown-content">
                  <Link to="/admin/members">
                    <span className="dropdown-icon">👥</span>
                    Members
                  </Link>
                  <Link to="/admin/courses">
                    <span className="dropdown-icon">📚</span>
                    Courses
                  </Link>
                  <Link to="/admin/schedules">
                    <span className="dropdown-icon">📅</span>
                    Schedules
                  </Link>
                  <Link to="/admin/announcements">
                    <span className="dropdown-icon">📢</span>
                    Announcements
                  </Link>
                  <Link to="/admin/blog">
                    <span className="dropdown-icon">📝</span>
                    Blog Posts
                  </Link>
                  <Link to="/admin/plans">
                    <span className="dropdown-icon">💳</span>
                    Membership Plans
                  </Link>
                </div>
              </div>
            )}
            <div className="nav-user">
              <div className="user-info">
                <span className="user-name">{user?.first_name} {user?.last_name}</span>
                <span className={`user-role ${user?.role}`}>{user?.role}</span>
              </div>
              <button onClick={handleLogout} className="btn-logout">Logout</button>
            </div>
          </>
        ) : (
          <div className="nav-auth">
            <Link to="/login" className="btn-login">Login</Link>
            <Link to="/register" className="btn-register">Join Now</Link>
          </div>
        )}
      </div>
    </nav>
  );
};

export default Navbar;
