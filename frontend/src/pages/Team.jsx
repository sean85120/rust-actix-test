import { useTranslation } from 'react-i18next';
import './Team.css';

const Team = () => {
  const { t } = useTranslation();

  return (
    <div className="team-page">
      <div className="page-header">
        <h1>{t('nav.team')}</h1>
        <p>{t('team.subtitle')}</p>
      </div>

      <div className="team-grid">
        <div className="team-member">
          <div className="member-photo">
            <div className="placeholder-avatar">JC</div>
          </div>
          <h3>John Chen</h3>
          <span className="member-role">{t('team.headCoach')}</span>
          <p>{t('team.johnBio')}</p>
        </div>

        <div className="team-member">
          <div className="member-photo">
            <div className="placeholder-avatar">ML</div>
          </div>
          <h3>Mike Lee</h3>
          <span className="member-role">{t('team.boxingCoach')}</span>
          <p>{t('team.mikeBio')}</p>
        </div>

        <div className="team-member">
          <div className="member-photo">
            <div className="placeholder-avatar">SW</div>
          </div>
          <h3>Sarah Wang</h3>
          <span className="member-role">{t('team.fitnessCoach')}</span>
          <p>{t('team.sarahBio')}</p>
        </div>

        <div className="team-member">
          <div className="member-photo">
            <div className="placeholder-avatar">DL</div>
          </div>
          <h3>David Lin</h3>
          <span className="member-role">{t('team.muayThaiCoach')}</span>
          <p>{t('team.davidBio')}</p>
        </div>
      </div>
    </div>
  );
};

export default Team;
