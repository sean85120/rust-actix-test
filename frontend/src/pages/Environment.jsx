import { useTranslation } from 'react-i18next';
import './Environment.css';

const Environment = () => {
  const { t } = useTranslation();

  return (
    <div className="environment-page">
      <div className="page-header">
        <h1>{t('nav.environment')}</h1>
        <p>{t('environment.subtitle')}</p>
      </div>

      <div className="facilities-grid">
        <div className="facility-card">
          <div className="facility-icon">🥊</div>
          <h3>{t('environment.boxingRing')}</h3>
          <p>{t('environment.boxingRingDesc')}</p>
        </div>

        <div className="facility-card">
          <div className="facility-icon">🏋️</div>
          <h3>{t('environment.weightRoom')}</h3>
          <p>{t('environment.weightRoomDesc')}</p>
        </div>

        <div className="facility-card">
          <div className="facility-icon">🎯</div>
          <h3>{t('environment.trainingArea')}</h3>
          <p>{t('environment.trainingAreaDesc')}</p>
        </div>

        <div className="facility-card">
          <div className="facility-icon">🚿</div>
          <h3>{t('environment.lockerRoom')}</h3>
          <p>{t('environment.lockerRoomDesc')}</p>
        </div>

        <div className="facility-card">
          <div className="facility-icon">🧘</div>
          <h3>{t('environment.stretchArea')}</h3>
          <p>{t('environment.stretchAreaDesc')}</p>
        </div>

        <div className="facility-card">
          <div className="facility-icon">☕</div>
          <h3>{t('environment.lounge')}</h3>
          <p>{t('environment.loungeDesc')}</p>
        </div>
      </div>

      <div className="environment-info">
        <div className="info-card">
          <h3>{t('environment.operatingHours')}</h3>
          <ul>
            <li>{t('environment.weekdayHours')}</li>
            <li>{t('environment.weekendHours')}</li>
          </ul>
        </div>

        <div className="info-card">
          <h3>{t('environment.location')}</h3>
          <p>{t('environment.address')}</p>
        </div>
      </div>
    </div>
  );
};

export default Environment;
