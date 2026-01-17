import { Outlet } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import Navbar from './Navbar';
import './Layout.css';

const Layout = () => {
  const { t } = useTranslation();

  return (
    <div className="layout">
      <Navbar />
      <main className="main-content">
        <Outlet />
      </main>
      <footer className="footer">
        <p>&copy; {t('common.copyright')}</p>
      </footer>
    </div>
  );
};

export default Layout;
