import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { blogApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminBlog = () => {
  const { t, i18n } = useTranslation();
  const [posts, setPosts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedPost, setSelectedPost] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [message, setMessage] = useState({ type: '', text: '' });

  const categories = ['news', 'training', 'nutrition', 'events', 'tips', 'stories'];

  const emptyPost = {
    title: '',
    content: '',
    excerpt: '',
    category: 'news',
    tags: '',
    is_featured: false,
    publish_immediately: false,
  };

  useEffect(() => {
    fetchPosts();
  }, []);

  const fetchPosts = async () => {
    try {
      setLoading(true);
      const response = await blogApi.adminList();
      setPosts(response.data.posts || []);
    } catch (error) {
      console.error('Error fetching posts:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setSelectedPost(emptyPost);
    setIsCreating(true);
    setIsModalOpen(true);
  };

  const handleEdit = (post) => {
    setSelectedPost({
      ...post,
      tags: Array.isArray(post.tags) ? post.tags.join(', ') : post.tags || '',
    });
    setIsCreating(false);
    setIsModalOpen(true);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const postData = {
        ...selectedPost,
        tags: selectedPost.tags
          ? selectedPost.tags.split(',').map((t) => t.trim()).filter((t) => t)
          : [],
      };

      if (isCreating) {
        await blogApi.create(postData);
        setMessage({ type: 'success', text: t('admin.postCreated') });
      } else {
        await blogApi.update(selectedPost.id, postData);
        setMessage({ type: 'success', text: t('admin.postUpdated') });
      }
      setIsModalOpen(false);
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.operationFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handlePublish = async (id) => {
    try {
      await blogApi.publish(id);
      setMessage({ type: 'success', text: t('admin.postPublished') });
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.publishFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleArchive = async (id) => {
    try {
      await blogApi.archive(id);
      setMessage({ type: 'success', text: t('admin.postArchived') });
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.archiveFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm(t('admin.deleteConfirmPost'))) return;
    try {
      await blogApi.delete(id);
      setMessage({ type: 'success', text: t('admin.postDeleted') });
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.deleteFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const formatDate = (dateStr) => {
    if (!dateStr) return '-';
    const locale = i18n.language === 'zh-TW' ? 'zh-TW' : 'en-US';
    return new Date(dateStr).toLocaleDateString(locale, {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  };

  const getCategoryLabel = (category) => {
    return t(`blog.categories.${category}`, { defaultValue: category });
  };

  const getStatusLabel = (status) => {
    return t(`admin.statuses.${status}`, { defaultValue: status });
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>{t('admin.blogPostsManagement')}</h1>
        <button className="btn-add" onClick={handleCreate}>{t('admin.addPost')}</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>{t('admin.title')}</th>
              <th>{t('admin.category')}</th>
              <th>{t('courses.status')}</th>
              <th>{t('admin.isFeatured')}</th>
              <th>{t('blog.views')}</th>
              <th>{t('admin.published')}</th>
              <th>{t('admin.actions')}</th>
            </tr>
          </thead>
          <tbody>
            {posts.length === 0 ? (
              <tr>
                <td colSpan="7" className="empty-state">{t('admin.noPostsFound')}</td>
              </tr>
            ) : (
              posts.map((post) => (
                <tr key={post.id}>
                  <td>{post.title}</td>
                  <td>{getCategoryLabel(post.category)}</td>
                  <td>
                    <span className={`status-badge ${post.status}`}>
                      {getStatusLabel(post.status)}
                    </span>
                  </td>
                  <td>{post.is_featured ? t('admin.yes') : t('admin.no')}</td>
                  <td>{post.view_count}</td>
                  <td>{formatDate(post.published_at)}</td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(post)}>
                        {t('admin.edit')}
                      </button>
                      {post.status === 'draft' && (
                        <button className="btn-action publish" onClick={() => handlePublish(post.id)}>
                          {t('admin.publish')}
                        </button>
                      )}
                      {post.status === 'published' && (
                        <button className="btn-action" onClick={() => handleArchive(post.id)}>
                          {t('admin.archive')}
                        </button>
                      )}
                      <button className="btn-action delete" onClick={() => handleDelete(post.id)}>
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
        title={isCreating ? t('admin.createBlogPost') : t('admin.editBlogPost')}
      >
        {selectedPost && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>{t('admin.title')}</label>
              <input
                type="text"
                value={selectedPost.title}
                onChange={(e) => setSelectedPost({ ...selectedPost, title: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>{t('admin.excerpt')}</label>
              <textarea
                value={selectedPost.excerpt}
                onChange={(e) => setSelectedPost({ ...selectedPost, excerpt: e.target.value })}
                placeholder={t('admin.excerptPlaceholder')}
                style={{ minHeight: '80px' }}
              />
            </div>
            <div className="form-group">
              <label>{t('admin.content')}</label>
              <textarea
                value={selectedPost.content}
                onChange={(e) => setSelectedPost({ ...selectedPost, content: e.target.value })}
                required
                style={{ minHeight: '200px' }}
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>{t('admin.category')}</label>
                <select
                  value={selectedPost.category}
                  onChange={(e) => setSelectedPost({ ...selectedPost, category: e.target.value })}
                >
                  {categories.map((category) => (
                    <option key={category} value={category}>
                      {getCategoryLabel(category)}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label>{t('admin.tags')}</label>
                <input
                  type="text"
                  value={selectedPost.tags}
                  onChange={(e) => setSelectedPost({ ...selectedPost, tags: e.target.value })}
                  placeholder={t('admin.tagsPlaceholder')}
                />
              </div>
            </div>
            <div className="form-group checkbox-group">
              <input
                type="checkbox"
                id="is_featured"
                checked={selectedPost.is_featured}
                onChange={(e) => setSelectedPost({ ...selectedPost, is_featured: e.target.checked })}
              />
              <label htmlFor="is_featured">{t('admin.featuredPost')}</label>
            </div>
            {isCreating && (
              <div className="form-group checkbox-group">
                <input
                  type="checkbox"
                  id="publish_immediately"
                  checked={selectedPost.publish_immediately}
                  onChange={(e) => setSelectedPost({ ...selectedPost, publish_immediately: e.target.checked })}
                />
                <label htmlFor="publish_immediately">{t('admin.publishImmediately')}</label>
              </div>
            )}
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

export default AdminBlog;
