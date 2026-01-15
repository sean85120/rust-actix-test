import { createContext, useContext, useState, useEffect } from 'react';
import { authApi } from '../api/client';

const AuthContext = createContext(null);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
};

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const token = localStorage.getItem('token');
    const savedUser = localStorage.getItem('user');

    if (token && savedUser) {
      setUser(JSON.parse(savedUser));
      // Verify token is still valid
      authApi.getCurrentUser()
        .then((res) => {
          setUser(res.data.member);
          localStorage.setItem('user', JSON.stringify(res.data.member));
        })
        .catch(() => {
          logout();
        })
        .finally(() => setLoading(false));
    } else {
      setLoading(false);
    }
  }, []);

  const login = async (email, password) => {
    const response = await authApi.login({ email, password });
    const { token, member } = response.data;
    localStorage.setItem('token', token);
    localStorage.setItem('user', JSON.stringify(member));
    setUser(member);
    return member;
  };

  const register = async (userData) => {
    const response = await authApi.register(userData);
    const { token, member } = response.data;
    localStorage.setItem('token', token);
    localStorage.setItem('user', JSON.stringify(member));
    setUser(member);
    return member;
  };

  const logout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    setUser(null);
  };

  const isAdmin = () => user?.role === 'admin';
  const isInstructor = () => user?.role === 'instructor' || user?.role === 'admin';

  return (
    <AuthContext.Provider value={{
      user,
      loading,
      login,
      register,
      logout,
      isAdmin,
      isInstructor,
      isAuthenticated: !!user,
    }}>
      {children}
    </AuthContext.Provider>
  );
};
