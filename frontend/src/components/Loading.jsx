import { useTranslation } from 'react-i18next';
import './Loading.css';

const Loading = ({ message }) => {
  const { t } = useTranslation();

  return (
    <div className="loading-container">
      <div className="loading-spinner"></div>
      <p>{message || t('common.loading')}</p>
    </div>
  );
};

export default Loading;
