import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { coursesApi } from '../api/client';
import Loading from '../components/Loading';
import './Courses.css';

const Courses = () => {
  const { t } = useTranslation();
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

  const getCourseTypeLabel = (type) => {
    return t(`courses.courseTypes.${type}`, { defaultValue: type.replace('_', ' ') });
  };

  const getDifficultyLabel = (level) => {
    return t(`courses.difficultyLevels.${level}`, { defaultValue: level });
  };

  return (
    <div className="courses-page">
      <div className="page-header">
        <h1>{t('courses.ourCourses')}</h1>
        <p>{t('courses.findPerfect')}</p>
      </div>

      <div className="filters">
        <select
          value={filters.course_type}
          onChange={(e) => setFilters({ ...filters, course_type: e.target.value })}
        >
          <option value="">{t('courses.allTypes')}</option>
          {courseTypes.map((type) => (
            <option key={type} value={type}>
              {getCourseTypeLabel(type)}
            </option>
          ))}
        </select>

        <select
          value={filters.difficulty_level}
          onChange={(e) => setFilters({ ...filters, difficulty_level: e.target.value })}
        >
          <option value="">{t('courses.allLevels')}</option>
          {difficultyLevels.map((level) => (
            <option key={level} value={level}>
              {getDifficultyLabel(level)}
            </option>
          ))}
        </select>
      </div>

      {loading ? (
        <Loading />
      ) : courses.length === 0 ? (
        <div className="no-results">
          <p>{t('courses.noResults')}</p>
        </div>
      ) : (
        <div className="courses-grid">
          {courses.map((course) => (
            <div key={course.id} className="course-card">
              <div className="course-header">
                <span className="course-type">{getCourseTypeLabel(course.course_type)}</span>
                <span className={`difficulty difficulty-${course.difficulty_level}`}>
                  {getDifficultyLabel(course.difficulty_level)}
                </span>
              </div>
              <h3>{course.name}</h3>
              <p className="course-description">{course.description}</p>
              <div className="course-details">
                <div className="detail">
                  <span className="label">{t('courses.duration')}</span>
                  <span className="value">{course.duration_minutes} {t('courses.min')}</span>
                </div>
                <div className="detail">
                  <span className="label">{t('courses.capacity')}</span>
                  <span className="value">{course.max_participants} {t('courses.people')}</span>
                </div>
              </div>
              <Link to={`/courses/${course.id}`} className="btn-view">
                {t('courses.viewDetails')}
              </Link>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Courses;
