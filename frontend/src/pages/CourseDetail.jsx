import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { coursesApi, schedulesApi, bookingsApi } from '../api/client';
import { useAuth } from '../context/AuthContext';
import Loading from '../components/Loading';
import './CourseDetail.css';

const CourseDetail = () => {
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
      setMessage({ type: 'success', text: 'Booking confirmed! Check your bookings for details.' });
      fetchCourseData();
    } catch (error) {
      setMessage({
        type: 'error',
        text: error.response?.data?.message || 'Failed to book class. Please try again.',
      });
    } finally {
      setBookingId(null);
    }
  };

  const formatDate = (dateStr) => {
    return new Date(dateStr).toLocaleDateString('en-US', {
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

  if (loading) return <Loading />;
  if (!course) return <div className="error">Course not found</div>;

  return (
    <div className="course-detail">
      <button onClick={() => navigate('/courses')} className="back-btn">
        &larr; Back to Courses
      </button>

      <div className="course-hero">
        <div className="course-badges">
          <span className="course-type">{course.course_type.replace('_', ' ')}</span>
          <span className={`difficulty difficulty-${course.difficulty_level}`}>
            {course.difficulty_level}
          </span>
        </div>
        <h1>{course.name}</h1>
        <p>{course.description}</p>
      </div>

      <div className="course-info-grid">
        <div className="info-card">
          <span className="info-label">Duration</span>
          <span className="info-value">{course.duration_minutes} minutes</span>
        </div>
        <div className="info-card">
          <span className="info-label">Max Participants</span>
          <span className="info-value">{course.max_participants} people</span>
        </div>
        <div className="info-card">
          <span className="info-label">Status</span>
          <span className={`info-value status-${course.is_active ? 'active' : 'inactive'}`}>
            {course.is_active ? 'Active' : 'Inactive'}
          </span>
        </div>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <section className="schedules-section">
        <h2>Upcoming Classes</h2>
        {schedules.length === 0 ? (
          <p className="no-schedules">No upcoming classes scheduled for this course.</p>
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
                      {schedule.max_participants - schedule.current_enrollment} spots left
                    </span>
                    <span className="spots-total">
                      of {schedule.max_participants}
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
                    ? 'Booking...'
                    : schedule.status === 'cancelled'
                    ? 'Cancelled'
                    : schedule.current_enrollment >= schedule.max_participants
                    ? 'Full'
                    : 'Book Now'}
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
