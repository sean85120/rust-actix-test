import { Link, useNavigate, useLocation } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { useAuth } from '../context/AuthContext';
import LanguageSwitcher from './LanguageSwitcher';
import './Navbar.css';

const Navbar = () => {
  const { t } = useTranslation();
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
          <span>Rockbird</span>
        </Link>
      </div>

      <div className="navbar-menu">
        <Link to="/" className={`nav-link ${isActive('/') ? 'active' : ''}`}>
          {t('nav.home')}
        </Link>
        <Link to="/courses" className={`nav-link ${isActive('/courses') ? 'active' : ''}`}>
          {t('nav.courses')}
        </Link>
        <Link to="/schedules" className={`nav-link ${isActive('/schedules') ? 'active' : ''}`}>
          {t('nav.schedule')}
        </Link>
        <Link to="/team" className={`nav-link ${isActive('/team') ? 'active' : ''}`}>
          {t('nav.team')}
        </Link>
        <Link to="/environment" className={`nav-link ${isActive('/environment') ? 'active' : ''}`}>
          {t('nav.environment')}
        </Link>
        <Link to="/blog" className={`nav-link ${isActive('/blog') ? 'active' : ''}`}>
          {t('nav.blog')}
        </Link>

        {isAuthenticated ? (
          <>
            <Link to="/my-bookings" className={`nav-link ${isActive('/my-bookings') ? 'active' : ''}`}>
              {t('nav.myBookings')}
            </Link>
            {isAdmin() && (
              <div className="nav-dropdown">
                <span className="nav-link dropdown-trigger">{t('nav.admin')}</span>
                <div className="dropdown-content">
                  <Link to="/admin/members">
                    <span className="dropdown-icon">👥</span>
                    {t('nav.members')}
                  </Link>
                  <Link to="/admin/courses">
                    <span className="dropdown-icon">📚</span>
                    {t('nav.courses')}
                  </Link>
                  <Link to="/admin/schedules">
                    <span className="dropdown-icon">📅</span>
                    {t('nav.schedules')}
                  </Link>
                  <Link to="/admin/announcements">
                    <span className="dropdown-icon">📢</span>
                    {t('nav.announcements')}
                  </Link>
                  <Link to="/admin/blog">
                    <span className="dropdown-icon">📝</span>
                    {t('nav.blogPosts')}
                  </Link>
                  <Link to="/admin/plans">
                    <span className="dropdown-icon">💳</span>
                    {t('nav.membershipPlans')}
                  </Link>
                </div>
              </div>
            )}
            <div className="nav-user">
              <div className="user-info">
                <span className="user-name">{user?.first_name} {user?.last_name}</span>
                <span className={`user-role ${user?.role}`}>{user?.role}</span>
              </div>
              <button onClick={handleLogout} className="btn-logout">{t('nav.logout')}</button>
            </div>
          </>
        ) : (
          <div className="nav-auth">
            <Link to="/login" className="btn-login">{t('nav.login')}</Link>
            <Link to="/register" className="btn-register">{t('nav.joinNow')}</Link>
          </div>
        )}
        <LanguageSwitcher />
      </div>
    </nav>
  );
};

export default Navbar;
