import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { coursesApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminCourses = () => {
  const { t } = useTranslation();
  const [courses, setCourses] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedCourse, setSelectedCourse] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [message, setMessage] = useState({ type: '', text: '' });

  const courseTypes = ['boxing', 'kickboxing', 'muay_thai', 'fitness', 'cardio', 'strength'];
  const difficultyLevels = ['beginner', 'intermediate', 'advanced'];

  const emptyCourse = {
    name: '',
    description: '',
    course_type: 'boxing',
    difficulty_level: 'beginner',
    duration_minutes: 60,
    max_participants: 20,
    is_active: true,
  };

  useEffect(() => {
    fetchCourses();
  }, []);

  const fetchCourses = async () => {
    try {
      setLoading(true);
      const response = await coursesApi.list();
      setCourses(response.data.courses || []);
    } catch (error) {
      console.error('Error fetching courses:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setSelectedCourse(emptyCourse);
    setIsCreating(true);
    setIsModalOpen(true);
  };

  const handleEdit = (course) => {
    setSelectedCourse(course);
    setIsCreating(false);
    setIsModalOpen(true);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isCreating) {
        await coursesApi.create(selectedCourse);
        setMessage({ type: 'success', text: t('admin.courseCreated') });
      } else {
        await coursesApi.update(selectedCourse.id, selectedCourse);
        setMessage({ type: 'success', text: t('admin.courseUpdated') });
      }
      setIsModalOpen(false);
      fetchCourses();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.operationFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm(t('admin.deleteConfirmCourse'))) return;
    try {
      await coursesApi.delete(id);
      setMessage({ type: 'success', text: t('admin.courseDeleted') });
      fetchCourses();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.deleteFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const getCourseTypeLabel = (type) => {
    return t(`courses.courseTypes.${type}`, { defaultValue: type.replace('_', ' ') });
  };

  const getDifficultyLabel = (level) => {
    return t(`courses.difficultyLevels.${level}`, { defaultValue: level });
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>{t('admin.coursesManagement')}</h1>
        <button className="btn-add" onClick={handleCreate}>{t('admin.addCourse')}</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>{t('admin.name')}</th>
              <th>{t('admin.type')}</th>
              <th>{t('admin.difficulty')}</th>
              <th>{t('courses.duration')}</th>
              <th>{t('courses.capacity')}</th>
              <th>{t('courses.status')}</th>
              <th>{t('admin.actions')}</th>
            </tr>
          </thead>
          <tbody>
            {courses.length === 0 ? (
              <tr>
                <td colSpan="7" className="empty-state">{t('admin.noCoursesFound')}</td>
              </tr>
            ) : (
              courses.map((course) => (
                <tr key={course.id}>
                  <td>{course.name}</td>
                  <td>{getCourseTypeLabel(course.course_type)}</td>
                  <td>
                    <span className={`difficulty difficulty-${course.difficulty_level}`}>
                      {getDifficultyLabel(course.difficulty_level)}
                    </span>
                  </td>
                  <td>{course.duration_minutes} {t('courses.min')}</td>
                  <td>{course.max_participants}</td>
                  <td>
                    <span className={`status-badge ${course.is_active ? 'active' : 'inactive'}`}>
                      {course.is_active ? t('courses.active') : t('courses.inactive')}
                    </span>
                  </td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(course)}>
                        {t('admin.edit')}
                      </button>
                      <button className="btn-action delete" onClick={() => handleDelete(course.id)}>
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
        title={isCreating ? t('admin.createCourse') : t('admin.editCourse')}
      >
        {selectedCourse && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>{t('admin.courseName')}</label>
              <input
                type="text"
                value={selectedCourse.name}
                onChange={(e) => setSelectedCourse({ ...selectedCourse, name: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>{t('admin.description')}</label>
              <textarea
                value={selectedCourse.description}
                onChange={(e) => setSelectedCourse({ ...selectedCourse, description: e.target.value })}
                required
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>{t('admin.type')}</label>
                <select
                  value={selectedCourse.course_type}
                  onChange={(e) => setSelectedCourse({ ...selectedCourse, course_type: e.target.value })}
                >
                  {courseTypes.map((type) => (
                    <option key={type} value={type}>
                      {getCourseTypeLabel(type)}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label>{t('admin.difficulty')}</label>
                <select
                  value={selectedCourse.difficulty_level}
                  onChange={(e) => setSelectedCourse({ ...selectedCourse, difficulty_level: e.target.value })}
                >
                  {difficultyLevels.map((level) => (
                    <option key={level} value={level}>
                      {getDifficultyLabel(level)}
                    </option>
                  ))}
                </select>
              </div>
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>{t('admin.durationMinutes')}</label>
                <input
                  type="number"
                  value={selectedCourse.duration_minutes}
                  onChange={(e) => setSelectedCourse({ ...selectedCourse, duration_minutes: parseInt(e.target.value) })}
                  min="15"
                  max="180"
                  required
                />
              </div>
              <div className="form-group">
                <label>{t('admin.maxParticipants')}</label>
                <input
                  type="number"
                  value={selectedCourse.max_participants}
                  onChange={(e) => setSelectedCourse({ ...selectedCourse, max_participants: parseInt(e.target.value) })}
                  min="1"
                  max="100"
                  required
                />
              </div>
            </div>
            <div className="form-group checkbox-group">
              <input
                type="checkbox"
                id="is_active"
                checked={selectedCourse.is_active}
                onChange={(e) => setSelectedCourse({ ...selectedCourse, is_active: e.target.checked })}
              />
              <label htmlFor="is_active">{t('courses.active')}</label>
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

export default AdminCourses;
