import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { coursesApi, schedulesApi, bookingsApi } from '../api/client';
import { useAuth } from '../context/AuthContext';
import Loading from '../components/Loading';
import './CourseDetail.css';

const CourseDetail = () => {
  const { t, i18n } = useTranslation();
  const { id } = useParams();
  const navigate = useNavigate();
  const { isAuthenticated } = useAuth();
  const [course, setCourse] = useState(null);
  const [schedules, setSchedules] = useState([]);
  const [loading, setLoading] = useState(true);
  const [bookingId, setBookingId] = useState(null);
  const [message, setMessage] = useState({ type: '', text: '' });

  useEffect(() => {
    fetchCourseData();
  }, [id]);

  const fetchCourseData = async () => {
    try {
      setLoading(true);
      const [courseRes, schedulesRes] = await Promise.all([
        coursesApi.getById(id),
        schedulesApi.list({ course_id: id }),
      ]);
      setCourse(courseRes.data);
      setSchedules(schedulesRes.data.schedules || []);
    } catch (error) {
      console.error('Error fetching course:', error);
      if (error.response?.status === 404) {
        navigate('/courses');
      }
    } finally {
      setLoading(false);
    }
  };

  const handleBook = async (scheduleId) => {
    if (!isAuthenticated) {
      navigate('/login');
      return;
    }

    try {
      setBookingId(scheduleId);
      await bookingsApi.create({ schedule_id: scheduleId });
      setMessage({ type: 'success', text: t('courses.bookingConfirmed') });
      fetchCourseData();
    } catch (error) {
      setMessage({
        type: 'error',
        text: error.response?.data?.message || t('courses.bookingFailed'),
      });
    } finally {
      setBookingId(null);
    }
  };

  const formatDate = (dateStr) => {
    const locale = i18n.language === 'zh-TW' ? 'zh-TW' : 'en-US';
    return new Date(dateStr).toLocaleDateString(locale, {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  const formatTime = (timeStr) => {
    const [hours, minutes] = timeStr.split(':');
    const hour = parseInt(hours);
    const ampm = hour >= 12 ? 'PM' : 'AM';
    const hour12 = hour % 12 || 12;
    return `${hour12}:${minutes} ${ampm}`;
  };

  const getCourseTypeLabel = (type) => {
    return t(`courses.courseTypes.${type}`, { defaultValue: type.replace('_', ' ') });
  };

  const getDifficultyLabel = (level) => {
    return t(`courses.difficultyLevels.${level}`, { defaultValue: level });
  };

  if (loading) return <Loading />;
  if (!course) return <div className="error">{t('courses.courseNotFound')}</div>;

  return (
    <div className="course-detail">
      <button onClick={() => navigate('/courses')} className="back-btn">
        &larr; {t('courses.backToCourses')}
      </button>

      <div className="course-hero">
        <div className="course-badges">
          <span className="course-type">{getCourseTypeLabel(course.course_type)}</span>
          <span className={`difficulty difficulty-${course.difficulty_level}`}>
            {getDifficultyLabel(course.difficulty_level)}
          </span>
        </div>
        <h1>{course.name}</h1>
        <p>{course.description}</p>
      </div>

      <div className="course-info-grid">
        <div className="info-card">
          <span className="info-label">{t('courses.duration')}</span>
          <span className="info-value">{course.duration_minutes} {t('courses.minutes')}</span>
        </div>
        <div className="info-card">
          <span className="info-label">{t('courses.maxParticipants')}</span>
          <span className="info-value">{course.max_participants} {t('courses.people')}</span>
        </div>
        <div className="info-card">
          <span className="info-label">{t('courses.status')}</span>
          <span className={`info-value status-${course.is_active ? 'active' : 'inactive'}`}>
            {course.is_active ? t('courses.active') : t('courses.inactive')}
          </span>
        </div>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <section className="schedules-section">
        <h2>{t('courses.upcomingClasses')}</h2>
        {schedules.length === 0 ? (
          <p className="no-schedules">{t('courses.noUpcoming')}</p>
        ) : (
          <div className="schedules-list">
            {schedules.map((schedule) => (
              <div key={schedule.id} className="schedule-card">
                <div className="schedule-date">
                  <span className="date">{formatDate(schedule.date)}</span>
                  <span className="time">
                    {formatTime(schedule.start_time)} - {formatTime(schedule.end_time)}
                  </span>
                </div>
                <div className="schedule-info">
                  <div className="spots">
                    <span className="spots-available">
                      {schedule.max_participants - schedule.current_enrollment} {t('courses.spotsLeft')}
                    </span>
                    <span className="spots-total">
                      {t('courses.of')} {schedule.max_participants}
                    </span>
                  </div>
                  <div className="progress-bar">
                    <div
                      className="progress"
                      style={{
                        width: `${(schedule.current_enrollment / schedule.max_participants) * 100}%`,
                      }}
                    />
                  </div>
                </div>
                <button
                  onClick={() => handleBook(schedule.id)}
                  disabled={
                    bookingId === schedule.id ||
                    schedule.current_enrollment >= schedule.max_participants ||
                    schedule.status === 'cancelled'
                  }
                  className="btn-book"
                >
                  {bookingId === schedule.id
                    ? t('courses.booking')
                    : schedule.status === 'cancelled'
                    ? t('courses.cancelled')
                    : schedule.current_enrollment >= schedule.max_participants
                    ? t('courses.full')
                    : t('courses.bookNow')}
                </button>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
};

export default CourseDetail;
