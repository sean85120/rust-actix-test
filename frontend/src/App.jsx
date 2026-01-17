import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './context/AuthContext';
import Layout from './components/Layout';
import Loading from './components/Loading';

// Public Pages
import Home from './pages/Home';
import Login from './pages/Login';
import Register from './pages/Register';
import Courses from './pages/Courses';
import CourseDetail from './pages/CourseDetail';
import Schedules from './pages/Schedules';
import Team from './pages/Team';
import Environment from './pages/Environment';
import Blog from './pages/Blog';
import BlogPost from './pages/BlogPost';
import MyBookings from './pages/MyBookings';

// Admin Pages
import AdminMembers from './pages/admin/AdminMembers';
import AdminCourses from './pages/admin/AdminCourses';
import AdminSchedules from './pages/admin/AdminSchedules';
import AdminAnnouncements from './pages/admin/AdminAnnouncements';
import AdminBlog from './pages/admin/AdminBlog';
import AdminPlans from './pages/admin/AdminPlans';

// Protected Route Component
const ProtectedRoute = ({ children, requireAdmin = false }) => {
  const { isAuthenticated, isAdmin, loading } = useAuth();

  if (loading) {
    return <Loading />;
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  if (requireAdmin && !isAdmin()) {
    return <Navigate to="/" replace />;
  }

  return children;
};

// Auth Route - redirects to home if already logged in
const AuthRoute = ({ children }) => {
  const { isAuthenticated, loading } = useAuth();

  if (loading) {
    return <Loading />;
  }

  if (isAuthenticated) {
    return <Navigate to="/" replace />;
  }

  return children;
};

function App() {
  return (
    <AuthProvider>
      <Router>
        <Routes>
          <Route path="/" element={<Layout />}>
            {/* Public Routes */}
            <Route index element={<Home />} />
            <Route path="courses" element={<Courses />} />
            <Route path="courses/:id" element={<CourseDetail />} />
            <Route path="schedules" element={<Schedules />} />
            <Route path="team" element={<Team />} />
            <Route path="environment" element={<Environment />} />
            <Route path="blog" element={<Blog />} />
            <Route path="blog/:slug" element={<BlogPost />} />

            {/* Auth Routes */}
            <Route
              path="login"
              element={
                <AuthRoute>
                  <Login />
                </AuthRoute>
              }
            />
            <Route
              path="register"
              element={
                <AuthRoute>
                  <Register />
                </AuthRoute>
              }
            />

            {/* Protected Routes */}
            <Route
              path="my-bookings"
              element={
                <ProtectedRoute>
                  <MyBookings />
                </ProtectedRoute>
              }
            />

            {/* Admin Routes */}
            <Route
              path="admin/members"
              element={
                <ProtectedRoute requireAdmin>
                  <AdminMembers />
                </ProtectedRoute>
              }
            />
            <Route
              path="admin/courses"
              element={
                <ProtectedRoute requireAdmin>
                  <AdminCourses />
                </ProtectedRoute>
              }
            />
            <Route
              path="admin/schedules"
              element={
                <ProtectedRoute requireAdmin>
                  <AdminSchedules />
                </ProtectedRoute>
              }
            />
            <Route
              path="admin/announcements"
              element={
                <ProtectedRoute requireAdmin>
                  <AdminAnnouncements />
                </ProtectedRoute>
              }
            />
            <Route
              path="admin/blog"
              element={
                <ProtectedRoute requireAdmin>
                  <AdminBlog />
                </ProtectedRoute>
              }
            />
            <Route
              path="admin/plans"
              element={
                <ProtectedRoute requireAdmin>
                  <AdminPlans />
                </ProtectedRoute>
              }
            />

            {/* Catch-all redirect */}
            <Route path="*" element={<Navigate to="/" replace />} />
          </Route>
        </Routes>
      </Router>
    </AuthProvider>
  );
}

export default App;
