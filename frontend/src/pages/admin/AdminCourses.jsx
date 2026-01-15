import { useState, useEffect } from 'react';
import { coursesApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminCourses = () => {
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
        setMessage({ type: 'success', text: 'Course created successfully!' });
      } else {
        await coursesApi.update(selectedCourse.id, selectedCourse);
        setMessage({ type: 'success', text: 'Course updated successfully!' });
      }
      setIsModalOpen(false);
      fetchCourses();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Operation failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm('Are you sure you want to delete this course?')) return;
    try {
      await coursesApi.delete(id);
      setMessage({ type: 'success', text: 'Course deleted successfully!' });
      fetchCourses();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Delete failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>Courses Management</h1>
        <button className="btn-add" onClick={handleCreate}>+ Add Course</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Type</th>
              <th>Difficulty</th>
              <th>Duration</th>
              <th>Capacity</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {courses.length === 0 ? (
              <tr>
                <td colSpan="7" className="empty-state">No courses found</td>
              </tr>
            ) : (
              courses.map((course) => (
                <tr key={course.id}>
                  <td>{course.name}</td>
                  <td>{course.course_type.replace('_', ' ')}</td>
                  <td>
                    <span className={`difficulty difficulty-${course.difficulty_level}`}>
                      {course.difficulty_level}
                    </span>
                  </td>
                  <td>{course.duration_minutes} min</td>
                  <td>{course.max_participants}</td>
                  <td>
                    <span className={`status-badge ${course.is_active ? 'active' : 'inactive'}`}>
                      {course.is_active ? 'Active' : 'Inactive'}
                    </span>
                  </td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(course)}>
                        Edit
                      </button>
                      <button className="btn-action delete" onClick={() => handleDelete(course.id)}>
                        Delete
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
        title={isCreating ? 'Create Course' : 'Edit Course'}
      >
        {selectedCourse && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>Course Name</label>
              <input
                type="text"
                value={selectedCourse.name}
                onChange={(e) => setSelectedCourse({ ...selectedCourse, name: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>Description</label>
              <textarea
                value={selectedCourse.description}
                onChange={(e) => setSelectedCourse({ ...selectedCourse, description: e.target.value })}
                required
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>Type</label>
                <select
                  value={selectedCourse.course_type}
                  onChange={(e) => setSelectedCourse({ ...selectedCourse, course_type: e.target.value })}
                >
                  {courseTypes.map((type) => (
                    <option key={type} value={type}>
                      {type.replace('_', ' ').replace(/\b\w/g, (l) => l.toUpperCase())}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label>Difficulty</label>
                <select
                  value={selectedCourse.difficulty_level}
                  onChange={(e) => setSelectedCourse({ ...selectedCourse, difficulty_level: e.target.value })}
                >
                  {difficultyLevels.map((level) => (
                    <option key={level} value={level}>
                      {level.charAt(0).toUpperCase() + level.slice(1)}
                    </option>
                  ))}
                </select>
              </div>
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>Duration (minutes)</label>
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
                <label>Max Participants</label>
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
              <label htmlFor="is_active">Active</label>
            </div>
            <div className="form-actions">
              <button type="button" className="btn-cancel-form" onClick={() => setIsModalOpen(false)}>
                Cancel
              </button>
              <button type="submit" className="btn-save">
                {isCreating ? 'Create' : 'Save Changes'}
              </button>
            </div>
          </form>
        )}
      </Modal>
    </div>
  );
};

export default AdminCourses;
