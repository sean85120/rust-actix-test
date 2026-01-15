import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { coursesApi } from '../api/client';
import Loading from '../components/Loading';
import './Courses.css';

const Courses = () => {
  const [courses, setCourses] = useState([]);
  const [loading, setLoading] = useState(true);
  const [filters, setFilters] = useState({
    course_type: '',
    difficulty_level: '',
  });

  useEffect(() => {
    fetchCourses();
  }, [filters]);

  const fetchCourses = async () => {
    try {
      setLoading(true);
      const params = {};
      if (filters.course_type) params.course_type = filters.course_type;
      if (filters.difficulty_level) params.difficulty_level = filters.difficulty_level;
      const response = await coursesApi.list(params);
      setCourses(response.data.courses || []);
    } catch (error) {
      console.error('Error fetching courses:', error);
    } finally {
      setLoading(false);
    }
  };

  const courseTypes = ['boxing', 'kickboxing', 'muay_thai', 'fitness', 'cardio', 'strength'];
  const difficultyLevels = ['beginner', 'intermediate', 'advanced'];

  return (
    <div className="courses-page">
      <div className="page-header">
        <h1>Our Courses</h1>
        <p>Find the perfect training program for your fitness goals</p>
      </div>

      <div className="filters">
        <select
          value={filters.course_type}
          onChange={(e) => setFilters({ ...filters, course_type: e.target.value })}
        >
          <option value="">All Types</option>
          {courseTypes.map((type) => (
            <option key={type} value={type}>
              {type.replace('_', ' ').replace(/\b\w/g, (l) => l.toUpperCase())}
            </option>
          ))}
        </select>

        <select
          value={filters.difficulty_level}
          onChange={(e) => setFilters({ ...filters, difficulty_level: e.target.value })}
        >
          <option value="">All Levels</option>
          {difficultyLevels.map((level) => (
            <option key={level} value={level}>
              {level.charAt(0).toUpperCase() + level.slice(1)}
            </option>
          ))}
        </select>
      </div>

      {loading ? (
        <Loading />
      ) : courses.length === 0 ? (
        <div className="no-results">
          <p>No courses found matching your criteria.</p>
        </div>
      ) : (
        <div className="courses-grid">
          {courses.map((course) => (
            <div key={course.id} className="course-card">
              <div className="course-header">
                <span className="course-type">{course.course_type.replace('_', ' ')}</span>
                <span className={`difficulty difficulty-${course.difficulty_level}`}>
                  {course.difficulty_level}
                </span>
              </div>
              <h3>{course.name}</h3>
              <p className="course-description">{course.description}</p>
              <div className="course-details">
                <div className="detail">
                  <span className="label">Duration</span>
                  <span className="value">{course.duration_minutes} min</span>
                </div>
                <div className="detail">
                  <span className="label">Capacity</span>
                  <span className="value">{course.max_participants} people</span>
                </div>
              </div>
              <Link to={`/courses/${course.id}`} className="btn-view">
                View Details
              </Link>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Courses;
