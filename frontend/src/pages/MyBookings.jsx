import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { bookingsApi } from '../api/client';
import Loading from '../components/Loading';
import './MyBookings.css';

const MyBookings = () => {
  const [bookings, setBookings] = useState([]);
  const [loading, setLoading] = useState(true);
  const [cancellingId, setCancellingId] = useState(null);
  const [message, setMessage] = useState({ type: '', text: '' });
  const [filter, setFilter] = useState('all');

  useEffect(() => {
    fetchBookings();
  }, []);

  const fetchBookings = async () => {
    try {
      setLoading(true);
      const response = await bookingsApi.getMyBookings();
      setBookings(response.data.bookings || []);
    } catch (error) {
      console.error('Error fetching bookings:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCancel = async (bookingId) => {
    if (!confirm('Are you sure you want to cancel this booking?')) return;

    try {
      setCancellingId(bookingId);
      await bookingsApi.cancel(bookingId);
      setMessage({ type: 'success', text: 'Booking cancelled successfully.' });
      fetchBookings();
    } catch (error) {
      setMessage({
        type: 'error',
        text: error.response?.data?.message || 'Failed to cancel booking.',
      });
    } finally {
      setCancellingId(null);
      setTimeout(() => setMessage({ type: '', text: '' }), 3000);
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
    if (!timeStr) return '';
    const [hours, minutes] = timeStr.split(':');
    const hour = parseInt(hours);
    const ampm = hour >= 12 ? 'PM' : 'AM';
    const hour12 = hour % 12 || 12;
    return `${hour12}:${minutes} ${ampm}`;
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'confirmed': return '#3498db';
      case 'attended': return '#2ecc71';
      case 'cancelled': return '#e74c3c';
      case 'no_show': return '#95a5a6';
      default: return '#888';
    }
  };

  const filteredBookings = bookings.filter((booking) => {
    if (filter === 'all') return true;
    if (filter === 'upcoming') return booking.status === 'confirmed';
    if (filter === 'past') return ['attended', 'no_show'].includes(booking.status);
    if (filter === 'cancelled') return booking.status === 'cancelled';
    return true;
  });

  return (
    <div className="my-bookings-page">
      <div className="page-header">
        <h1>My Bookings</h1>
        <p>Manage your class reservations</p>
      </div>

      <div className="filters">
        <button
          className={`filter-btn ${filter === 'all' ? 'active' : ''}`}
          onClick={() => setFilter('all')}
        >
          All
        </button>
        <button
          className={`filter-btn ${filter === 'upcoming' ? 'active' : ''}`}
          onClick={() => setFilter('upcoming')}
        >
          Upcoming
        </button>
        <button
          className={`filter-btn ${filter === 'past' ? 'active' : ''}`}
          onClick={() => setFilter('past')}
        >
          Past
        </button>
        <button
          className={`filter-btn ${filter === 'cancelled' ? 'active' : ''}`}
          onClick={() => setFilter('cancelled')}
        >
          Cancelled
        </button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      {loading ? (
        <Loading />
      ) : filteredBookings.length === 0 ? (
        <div className="no-bookings">
          <p>No bookings found.</p>
          <Link to="/schedules" className="btn-primary">Browse Classes</Link>
        </div>
      ) : (
        <div className="bookings-list">
          {filteredBookings.map((booking) => (
            <div key={booking.id} className="booking-card">
              <div className="booking-status" style={{ backgroundColor: getStatusColor(booking.status) }}>
                {booking.status.replace('_', ' ')}
              </div>
              <div className="booking-content">
                <div className="booking-main">
                  <Link to={`/courses/${booking.course_id}`} className="course-name">
                    {booking.course_name || 'Course'}
                  </Link>
                  <div className="booking-datetime">
                    <span className="date">{formatDate(booking.schedule_date)}</span>
                    <span className="time">
                      {formatTime(booking.start_time)} - {formatTime(booking.end_time)}
                    </span>
                  </div>
                  {booking.instructor_name && (
                    <span className="instructor">Instructor: {booking.instructor_name}</span>
                  )}
                </div>
                <div className="booking-actions">
                  {booking.status === 'confirmed' && (
                    <button
                      onClick={() => handleCancel(booking.id)}
                      disabled={cancellingId === booking.id}
                      className="btn-cancel"
                    >
                      {cancellingId === booking.id ? 'Cancelling...' : 'Cancel'}
                    </button>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default MyBookings;
