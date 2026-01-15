import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { schedulesApi, bookingsApi } from '../api/client';
import { useAuth } from '../context/AuthContext';
import Loading from '../components/Loading';
import './Schedules.css';

const Schedules = () => {
  const navigate = useNavigate();
  const { isAuthenticated } = useAuth();
  const [schedules, setSchedules] = useState([]);
  const [loading, setLoading] = useState(true);
  const [bookingId, setBookingId] = useState(null);
  const [message, setMessage] = useState({ type: '', text: '' });
  const [dateFilter, setDateFilter] = useState('');

  useEffect(() => {
    fetchSchedules();
  }, [dateFilter]);

  const fetchSchedules = async () => {
    try {
      setLoading(true);
      const params = { status: 'scheduled' };
      if (dateFilter) params.date = dateFilter;
      const response = await schedulesApi.list(params);
      setSchedules(response.data.schedules || []);
    } catch (error) {
      console.error('Error fetching schedules:', error);
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
      setMessage({ type: 'success', text: 'Booking confirmed!' });
      fetchSchedules();
    } catch (error) {
      setMessage({
        type: 'error',
        text: error.response?.data?.message || 'Failed to book. Please try again.',
      });
    } finally {
      setBookingId(null);
      setTimeout(() => setMessage({ type: '', text: '' }), 3000);
    }
  };

  const formatDate = (dateStr) => {
    return new Date(dateStr).toLocaleDateString('en-US', {
      weekday: 'short',
      month: 'short',
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

  // Group schedules by date
  const groupedSchedules = schedules.reduce((acc, schedule) => {
    const date = schedule.date;
    if (!acc[date]) acc[date] = [];
    acc[date].push(schedule);
    return acc;
  }, {});

  return (
    <div className="schedules-page">
      <div className="page-header">
        <h1>Class Schedule</h1>
        <p>View and book upcoming classes</p>
      </div>

      <div className="filters">
        <input
          type="date"
          value={dateFilter}
          onChange={(e) => setDateFilter(e.target.value)}
          className="date-filter"
        />
        {dateFilter && (
          <button onClick={() => setDateFilter('')} className="clear-filter">
            Clear
          </button>
        )}
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      {loading ? (
        <Loading />
      ) : Object.keys(groupedSchedules).length === 0 ? (
        <div className="no-results">
          <p>No classes scheduled.</p>
        </div>
      ) : (
        <div className="schedule-groups">
          {Object.entries(groupedSchedules)
            .sort(([a], [b]) => a.localeCompare(b))
            .map(([date, daySchedules]) => (
              <div key={date} className="schedule-group">
                <h2 className="date-header">{formatDate(date)}</h2>
                <div className="day-schedules">
                  {daySchedules.map((schedule) => (
                    <div key={schedule.id} className="schedule-item">
                      <div className="schedule-time">
                        {formatTime(schedule.start_time)} - {formatTime(schedule.end_time)}
                      </div>
                      <div className="schedule-details">
                        <Link to={`/courses/${schedule.course_id}`} className="course-name">
                          {schedule.course_name || 'Course'}
                        </Link>
                        <span className="instructor">
                          with {schedule.instructor_name || 'Instructor'}
                        </span>
                      </div>
                      <div className="schedule-capacity">
                        <span className="spots">
                          {schedule.max_participants - schedule.current_enrollment} / {schedule.max_participants}
                        </span>
                        <span className="label">spots</span>
                      </div>
                      <button
                        onClick={() => handleBook(schedule.id)}
                        disabled={
                          bookingId === schedule.id ||
                          schedule.current_enrollment >= schedule.max_participants
                        }
                        className="btn-book"
                      >
                        {bookingId === schedule.id
                          ? 'Booking...'
                          : schedule.current_enrollment >= schedule.max_participants
                          ? 'Full'
                          : 'Book'}
                      </button>
                    </div>
                  ))}
                </div>
              </div>
            ))}
        </div>
      )}
    </div>
  );
};

export default Schedules;
