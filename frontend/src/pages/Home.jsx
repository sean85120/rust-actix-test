import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { announcementsApi, coursesApi, blogApi } from '../api/client';
import { useAuth } from '../context/AuthContext';
import Loading from '../components/Loading';
import './Home.css';

const Home = () => {
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

  if (loading) return <Loading />;

  return (
    <div className="home">
      {/* Hero Section */}
      <section className="hero">
        <div className="hero-content">
          <h1>
            Train Like
            <span>A Champion</span>
          </h1>
          <p>
            Unleash your potential with world-class boxing training.
            Expert coaches, premium facilities, and a community that pushes you to greatness.
          </p>
          {!isAuthenticated && (
            <div className="hero-buttons">
              <Link to="/register" className="btn btn-primary">Start Training</Link>
              <Link to="/courses" className="btn btn-secondary">Explore Classes</Link>
            </div>
          )}
        </div>
      </section>

      {/* Stats Bar */}
      <div className="stats-bar">
        <div className="stat-item">
          <div className="stat-number">500+</div>
          <div className="stat-label">Active Members</div>
        </div>
        <div className="stat-item">
          <div className="stat-number">15+</div>
          <div className="stat-label">Expert Trainers</div>
        </div>
        <div className="stat-item">
          <div className="stat-number">50+</div>
          <div className="stat-label">Weekly Classes</div>
        </div>
        <div className="stat-item">
          <div className="stat-number">10+</div>
          <div className="stat-label">Years Experience</div>
        </div>
      </div>

      {/* Announcements */}
      {announcements.length > 0 && (
        <section className="announcements-section">
          <h2>Announcements</h2>
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
          <h2>Popular Courses</h2>
          <Link to="/courses" className="view-all">View All →</Link>
        </div>
        <div className="courses-grid">
          {courses.map((course) => (
            <div key={course.id} className="course-card">
              <div className="course-type">{course.course_type.replace('_', ' ')}</div>
              <h3>{course.name}</h3>
              <p>{course.description}</p>
              <div className="course-meta">
                <span className={`difficulty difficulty-${course.difficulty_level}`}>
                  {course.difficulty_level}
                </span>
                <span className="duration">{course.duration_minutes} min</span>
              </div>
              <Link to={`/courses/${course.id}`} className="btn btn-outline">
                Learn More
              </Link>
            </div>
          ))}
        </div>
      </section>

      {/* Recent Blog Posts */}
      {recentPosts.length > 0 && (
        <section className="blog-section">
          <div className="section-header">
            <h2>Latest from the Blog</h2>
            <Link to="/blog" className="view-all">View All →</Link>
          </div>
          <div className="blog-grid">
            {recentPosts.map((post) => (
              <div key={post.id} className="blog-card">
                <span className="blog-category">{post.category}</span>
                <h3>{post.title}</h3>
                <p>{post.excerpt}</p>
                <Link to={`/blog/${post.slug}`} className="read-more">
                  Read More →
                </Link>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* Features Section */}
      <section className="features-section">
        <h2>Why Choose <span>Knockout</span>?</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">🥊</div>
            <h3>Expert Trainers</h3>
            <p>Learn from professional boxers and certified fitness instructors with years of competitive experience.</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🏆</div>
            <h3>Premium Equipment</h3>
            <p>Train with top-tier boxing equipment, professional rings, and state-of-the-art fitness machines.</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📅</div>
            <h3>Flexible Schedule</h3>
            <p>Choose from a wide range of classes throughout the week that fit perfectly into your lifestyle.</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💪</div>
            <h3>Strong Community</h3>
            <p>Join a supportive community of boxing enthusiasts who motivate and push each other to excel.</p>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Home;
