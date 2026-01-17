import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { schedulesApi, coursesApi, membersApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminSchedules = () => {
  const { t, i18n } = useTranslation();
  const [schedules, setSchedules] = useState([]);
  const [courses, setCourses] = useState([]);
  const [instructors, setInstructors] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedSchedule, setSelectedSchedule] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [message, setMessage] = useState({ type: '', text: '' });

  const emptySchedule = {
    course_id: '',
    instructor_id: '',
    date: '',
    start_time: '09:00',
    end_time: '10:00',
    max_participants: 20,
  };

  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    try {
      setLoading(true);
      const [schedulesRes, coursesRes, membersRes] = await Promise.all([
        schedulesApi.list(),
        coursesApi.list(),
        membersApi.list(),
      ]);
      setSchedules(schedulesRes.data.schedules || []);
      setCourses(coursesRes.data.courses || []);
      // Filter for instructors and admins
      setInstructors(
        (membersRes.data.members || []).filter(
          (m) => m.role === 'instructor' || m.role === 'admin'
        )
      );
    } catch (error) {
      console.error('Error fetching data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setSelectedSchedule({
      ...emptySchedule,
      course_id: courses[0]?.id || '',
      instructor_id: instructors[0]?.id || '',
    });
    setIsCreating(true);
    setIsModalOpen(true);
  };

  const handleEdit = (schedule) => {
    setSelectedSchedule(schedule);
    setIsCreating(false);
    setIsModalOpen(true);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isCreating) {
        await schedulesApi.create(selectedSchedule);
        setMessage({ type: 'success', text: t('admin.scheduleCreated') });
      } else {
        await schedulesApi.update(selectedSchedule.id, selectedSchedule);
        setMessage({ type: 'success', text: t('admin.scheduleUpdated') });
      }
      setIsModalOpen(false);
      fetchData();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.operationFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleCancel = async (id) => {
    if (!confirm(t('admin.cancelConfirmSchedule'))) return;
    try {
      await schedulesApi.cancel(id);
      setMessage({ type: 'success', text: t('admin.scheduleCancelled') });
      fetchData();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.cancelFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm(t('admin.deleteConfirmSchedule'))) return;
    try {
      await schedulesApi.delete(id);
      setMessage({ type: 'success', text: t('admin.scheduleDeleted') });
      fetchData();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.deleteFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const formatDate = (dateStr) => {
    const locale = i18n.language === 'zh-TW' ? 'zh-TW' : 'en-US';
    return new Date(dateStr).toLocaleDateString(locale, {
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

  const getStatusLabel = (status) => {
    return t(`admin.statuses.${status}`, { defaultValue: status });
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>{t('admin.schedulesManagement')}</h1>
        <button className="btn-add" onClick={handleCreate}>{t('admin.addSchedule')}</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>{t('admin.date')}</th>
              <th>{t('admin.time')}</th>
              <th>{t('admin.course')}</th>
              <th>{t('admin.instructor')}</th>
              <th>{t('admin.enrollment')}</th>
              <th>{t('courses.status')}</th>
              <th>{t('admin.actions')}</th>
            </tr>
          </thead>
          <tbody>
            {schedules.length === 0 ? (
              <tr>
                <td colSpan="7" className="empty-state">{t('admin.noSchedulesFound')}</td>
              </tr>
            ) : (
              schedules.map((schedule) => (
                <tr key={schedule.id}>
                  <td>{formatDate(schedule.date)}</td>
                  <td>{formatTime(schedule.start_time)} - {formatTime(schedule.end_time)}</td>
                  <td>{schedule.course_name || 'N/A'}</td>
                  <td>{schedule.instructor_name || 'N/A'}</td>
                  <td>{schedule.current_enrollment} / {schedule.max_participants}</td>
                  <td>
                    <span className={`status-badge ${schedule.status}`}>
                      {getStatusLabel(schedule.status)}
                    </span>
                  </td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(schedule)}>
                        {t('admin.edit')}
                      </button>
                      {schedule.status === 'scheduled' && (
                        <button className="btn-action" onClick={() => handleCancel(schedule.id)}>
                          {t('admin.cancel')}
                        </button>
                      )}
                      <button className="btn-action delete" onClick={() => handleDelete(schedule.id)}>
                        {t('admin.delete')}
                      </button>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      <Modal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        title={isCreating ? t('admin.createSchedule') : t('admin.editSchedule')}
      >
        {selectedSchedule && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>{t('admin.course')}</label>
              <select
                value={selectedSchedule.course_id}
                onChange={(e) => setSelectedSchedule({ ...selectedSchedule, course_id: e.target.value })}
                required
              >
                <option value="">{t('admin.selectCourse')}</option>
                {courses.map((course) => (
                  <option key={course.id} value={course.id}>{course.name}</option>
                ))}
              </select>
            </div>
            <div className="form-group">
              <label>{t('admin.instructor')}</label>
              <select
                value={selectedSchedule.instructor_id}
                onChange={(e) => setSelectedSchedule({ ...selectedSchedule, instructor_id: e.target.value })}
                required
              >
                <option value="">{t('admin.selectInstructor')}</option>
                {instructors.map((instructor) => (
                  <option key={instructor.id} value={instructor.id}>
                    {instructor.first_name} {instructor.last_name}
                  </option>
                ))}
              </select>
            </div>
            <div className="form-group">
              <label>{t('admin.date')}</label>
              <input
                type="date"
                value={selectedSchedule.date}
                onChange={(e) => setSelectedSchedule({ ...selectedSchedule, date: e.target.value })}
                required
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>{t('admin.startTime')}</label>
                <input
                  type="time"
                  value={selectedSchedule.start_time}
                  onChange={(e) => setSelectedSchedule({ ...selectedSchedule, start_time: e.target.value })}
                  required
                />
              </div>
              <div className="form-group">
                <label>{t('admin.endTime')}</label>
                <input
                  type="time"
                  value={selectedSchedule.end_time}
                  onChange={(e) => setSelectedSchedule({ ...selectedSchedule, end_time: e.target.value })}
                  required
                />
              </div>
            </div>
            <div className="form-group">
              <label>{t('admin.maxParticipants')}</label>
              <input
                type="number"
                value={selectedSchedule.max_participants}
                onChange={(e) => setSelectedSchedule({ ...selectedSchedule, max_participants: parseInt(e.target.value) })}
                min="1"
                max="100"
                required
              />
            </div>
            <div className="form-actions">
              <button type="button" className="btn-cancel-form" onClick={() => setIsModalOpen(false)}>
                {t('admin.cancel')}
              </button>
              <button type="submit" className="btn-save">
                {isCreating ? t('admin.create') : t('admin.saveChanges')}
              </button>
            </div>
          </form>
        )}
      </Modal>
    </div>
  );
};

export default AdminSchedules;
