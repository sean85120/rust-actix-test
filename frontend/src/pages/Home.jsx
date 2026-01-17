import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { announcementsApi, coursesApi, blogApi } from '../api/client';
import { useAuth } from '../context/AuthContext';
import Loading from '../components/Loading';
import './Home.css';

const Home = () => {
  const { t } = useTranslation();
  const { isAuthenticated } = useAuth();
  const [announcements, setAnnouncements] = useState([]);
  const [courses, setCourses] = useState([]);
  const [recentPosts, setRecentPosts] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [announcementsRes, coursesRes, postsRes] = await Promise.all([
          announcementsApi.getActive().catch(() => ({ data: { announcements: [] } })),
          coursesApi.list({ limit: 6 }).catch(() => ({ data: { courses: [] } })),
          blogApi.getRecent(3).catch(() => ({ data: { posts: [] } })),
        ]);
        setAnnouncements(announcementsRes.data.announcements || []);
        setCourses(coursesRes.data.courses || []);
        setRecentPosts(postsRes.data.posts || []);
      } catch (error) {
        console.error('Error fetching data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  const getCourseTypeLabel = (type) => {
    return t(`courses.courseTypes.${type}`, { defaultValue: type.replace('_', ' ') });
  };

  const getDifficultyLabel = (level) => {
    return t(`courses.difficultyLevels.${level}`, { defaultValue: level });
  };

  if (loading) return <Loading />;

  return (
    <div className="home">
      {/* Hero Section */}
      <section className="hero">
        <div className="hero-content">
          <h1>
            {t('home.trainLike')}
            <span>{t('home.aChampion')}</span>
          </h1>
          <p>{t('home.heroDescription')}</p>
          {!isAuthenticated && (
            <div className="hero-buttons">
              <Link to="/register" className="btn btn-primary">{t('home.startTraining')}</Link>
              <Link to="/courses" className="btn btn-secondary">{t('home.exploreClasses')}</Link>
            </div>
          )}
        </div>
      </section>

      {/* Stats Bar */}
      <div className="stats-bar">
        <div className="stat-item">
          <div className="stat-number">500+</div>
          <div className="stat-label">{t('home.activeMembers')}</div>
        </div>
        <div className="stat-item">
          <div className="stat-number">15+</div>
          <div className="stat-label">{t('home.expertTrainers')}</div>
        </div>
        <div className="stat-item">
          <div className="stat-number">50+</div>
          <div className="stat-label">{t('home.weeklyClasses')}</div>
        </div>
        <div className="stat-item">
          <div className="stat-number">10+</div>
          <div className="stat-label">{t('home.yearsExperience')}</div>
        </div>
      </div>

      {/* Announcements */}
      {announcements.length > 0 && (
        <section className="announcements-section">
          <h2>{t('home.announcements')}</h2>
          <div className="announcements-list">
            {announcements.map((announcement) => (
              <div
                key={announcement.id}
                className={`announcement-card priority-${announcement.priority}`}
              >
                <span className="announcement-priority">{announcement.priority}</span>
                <h3>{announcement.title}</h3>
                <p>{announcement.content}</p>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Featured Courses */}
      <section className="courses-section">
        <div className="section-header">
          <h2>{t('home.popularCourses')}</h2>
          <Link to="/courses" className="view-all">{t('home.viewAll')} →</Link>
        </div>
        <div className="courses-grid">
          {courses.map((course) => (
            <div key={course.id} className="course-card">
              <div className="course-type">{getCourseTypeLabel(course.course_type)}</div>
              <h3>{course.name}</h3>
              <p>{course.description}</p>
              <div className="course-meta">
                <span className={`difficulty difficulty-${course.difficulty_level}`}>
                  {getDifficultyLabel(course.difficulty_level)}
                </span>
                <span className="duration">{course.duration_minutes} {t('courses.min')}</span>
              </div>
              <Link to={`/courses/${course.id}`} className="btn btn-outline">
                {t('home.learnMore')}
              </Link>
            </div>
          ))}
        </div>
      </section>

      {/* Recent Blog Posts */}
      {recentPosts.length > 0 && (
        <section className="blog-section">
          <div className="section-header">
            <h2>{t('home.latestFromBlog')}</h2>
            <Link to="/blog" className="view-all">{t('home.viewAll')} →</Link>
          </div>
          <div className="blog-grid">
            {recentPosts.map((post) => (
              <div key={post.id} className="blog-card">
                <span className="blog-category">{post.category}</span>
                <h3>{post.title}</h3>
                <p>{post.excerpt}</p>
                <Link to={`/blog/${post.slug}`} className="read-more">
                  {t('home.readMore')} →
                </Link>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Features Section */}
      <section className="features-section">
        <h2>{t('home.whyChoose')} <span>{t('home.knockout')}</span>?</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">🥊</div>
            <h3>{t('home.expertTrainersTitle')}</h3>
            <p>{t('home.expertTrainersDesc')}</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🏆</div>
            <h3>{t('home.premiumEquipment')}</h3>
            <p>{t('home.premiumEquipmentDesc')}</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📅</div>
            <h3>{t('home.flexibleSchedule')}</h3>
            <p>{t('home.flexibleScheduleDesc')}</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💪</div>
            <h3>{t('home.strongCommunity')}</h3>
            <p>{t('home.strongCommunityDesc')}</p>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Home;
