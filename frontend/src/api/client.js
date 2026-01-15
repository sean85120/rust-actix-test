import axios from 'axios';

// Use relative URL for Vite proxy in development, or full URL in production
const API_BASE_URL = '/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Handle auth errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Auth API
export const authApi = {
  register: (data) => api.post('/auth/register', data),
  login: (data) => api.post('/auth/login', data),
  getCurrentUser: () => api.get('/auth/me'),
};

// Members API
export const membersApi = {
  list: (params) => api.get('/members', { params }),
  getById: (id) => api.get(`/members/${id}`),
  update: (id, data) => api.put(`/members/${id}`, data),
  delete: (id) => api.delete(`/members/${id}`),
};

// Courses API
export const coursesApi = {
  list: (params) => api.get('/courses', { params }),
  getById: (id) => api.get(`/courses/${id}`),
  create: (data) => api.post('/courses', data),
  update: (id, data) => api.put(`/courses/${id}`, data),
  delete: (id) => api.delete(`/courses/${id}`),
};

// Schedules API
export const schedulesApi = {
  list: (params) => api.get('/schedules', { params }),
  getById: (id) => api.get(`/schedules/${id}`),
  create: (data) => api.post('/schedules', data),
  update: (id, data) => api.put(`/schedules/${id}`, data),
  delete: (id) => api.delete(`/schedules/${id}`),
  cancel: (id) => api.post(`/schedules/${id}/cancel`),
};

// Bookings API
export const bookingsApi = {
  list: (params) => api.get('/bookings', { params }),
  getMyBookings: () => api.get('/bookings/my'),
  create: (data) => api.post('/bookings', data),
  cancel: (id) => api.post(`/bookings/${id}/cancel`),
  attend: (id) => api.post(`/bookings/${id}/attend`),
};

// Membership Plans API
export const plansApi = {
  list: () => api.get('/membership-plans'),
  getById: (id) => api.get(`/membership-plans/${id}`),
  create: (data) => api.post('/membership-plans', data),
  update: (id, data) => api.put(`/membership-plans/${id}`, data),
  delete: (id) => api.delete(`/membership-plans/${id}`),
};

// Announcements API
export const announcementsApi = {
  // Admin endpoints
  list: (params) => api.get('/admin/announcements', { params }),
  getById: (id) => api.get(`/admin/announcements/${id}`),
  create: (data) => api.post('/admin/announcements', data),
  update: (id, data) => api.put(`/admin/announcements/${id}`, data),
  delete: (id) => api.delete(`/admin/announcements/${id}`),
  publish: (id) => api.post(`/admin/announcements/${id}/publish`),
  archive: (id) => api.post(`/admin/announcements/${id}/archive`),
  // Public endpoints
  getActive: () => api.get('/announcements/active'),
};

// Blog API
export const blogApi = {
  // Admin endpoints
  adminList: (params) => api.get('/admin/blog', { params }),
  adminGetById: (id) => api.get(`/admin/blog/${id}`),
  create: (data) => api.post('/admin/blog', data),
  update: (id, data) => api.put(`/admin/blog/${id}`, data),
  delete: (id) => api.delete(`/admin/blog/${id}`),
  publish: (id) => api.post(`/admin/blog/${id}/publish`),
  archive: (id) => api.post(`/admin/blog/${id}/archive`),
  // Public endpoints
  list: (params) => api.get('/blog', { params }),
  getBySlug: (slug) => api.get(`/blog/${slug}`),
  getFeatured: () => api.get('/blog/featured'),
  getRecent: (limit) => api.get('/blog/recent', { params: { limit } }),
};

export default api;
