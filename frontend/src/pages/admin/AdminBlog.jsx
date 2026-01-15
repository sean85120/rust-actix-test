import { useState, useEffect } from 'react';
import { blogApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminBlog = () => {
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
        setMessage({ type: 'success', text: 'Blog post created successfully!' });
      } else {
        await blogApi.update(selectedPost.id, postData);
        setMessage({ type: 'success', text: 'Blog post updated successfully!' });
      }
      setIsModalOpen(false);
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Operation failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handlePublish = async (id) => {
    try {
      await blogApi.publish(id);
      setMessage({ type: 'success', text: 'Blog post published!' });
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Publish failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleArchive = async (id) => {
    try {
      await blogApi.archive(id);
      setMessage({ type: 'success', text: 'Blog post archived!' });
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Archive failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm('Are you sure you want to delete this blog post?')) return;
    try {
      await blogApi.delete(id);
      setMessage({ type: 'success', text: 'Blog post deleted!' });
      fetchPosts();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Delete failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const formatDate = (dateStr) => {
    if (!dateStr) return '-';
    return new Date(dateStr).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>Blog Posts Management</h1>
        <button className="btn-add" onClick={handleCreate}>+ Add Post</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>Title</th>
              <th>Category</th>
              <th>Status</th>
              <th>Featured</th>
              <th>Views</th>
              <th>Published</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {posts.length === 0 ? (
              <tr>
                <td colSpan="7" className="empty-state">No blog posts found</td>
              </tr>
            ) : (
              posts.map((post) => (
                <tr key={post.id}>
                  <td>{post.title}</td>
                  <td>{post.category}</td>
                  <td>
                    <span className={`status-badge ${post.status}`}>
                      {post.status}
                    </span>
                  </td>
                  <td>{post.is_featured ? 'Yes' : 'No'}</td>
                  <td>{post.view_count}</td>
                  <td>{formatDate(post.published_at)}</td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(post)}>
                        Edit
                      </button>
                      {post.status === 'draft' && (
                        <button className="btn-action publish" onClick={() => handlePublish(post.id)}>
                          Publish
                        </button>
                      )}
                      {post.status === 'published' && (
                        <button className="btn-action" onClick={() => handleArchive(post.id)}>
                          Archive
                        </button>
                      )}
                      <button className="btn-action delete" onClick={() => handleDelete(post.id)}>
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
        title={isCreating ? 'Create Blog Post' : 'Edit Blog Post'}
      >
        {selectedPost && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>Title</label>
              <input
                type="text"
                value={selectedPost.title}
                onChange={(e) => setSelectedPost({ ...selectedPost, title: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>Excerpt</label>
              <textarea
                value={selectedPost.excerpt}
                onChange={(e) => setSelectedPost({ ...selectedPost, excerpt: e.target.value })}
                placeholder="Brief summary of the post"
                style={{ minHeight: '80px' }}
              />
            </div>
            <div className="form-group">
              <label>Content</label>
              <textarea
                value={selectedPost.content}
                onChange={(e) => setSelectedPost({ ...selectedPost, content: e.target.value })}
                required
                style={{ minHeight: '200px' }}
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>Category</label>
                <select
                  value={selectedPost.category}
                  onChange={(e) => setSelectedPost({ ...selectedPost, category: e.target.value })}
                >
                  {categories.map((category) => (
                    <option key={category} value={category}>
                      {category.charAt(0).toUpperCase() + category.slice(1)}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label>Tags (comma-separated)</label>
                <input
                  type="text"
                  value={selectedPost.tags}
                  onChange={(e) => setSelectedPost({ ...selectedPost, tags: e.target.value })}
                  placeholder="boxing, fitness, tips"
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
              <label htmlFor="is_featured">Featured Post</label>
            </div>
            {isCreating && (
              <div className="form-group checkbox-group">
                <input
                  type="checkbox"
                  id="publish_immediately"
                  checked={selectedPost.publish_immediately}
                  onChange={(e) => setSelectedPost({ ...selectedPost, publish_immediately: e.target.checked })}
                />
                <label htmlFor="publish_immediately">Publish Immediately</label>
              </div>
            )}
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

export default AdminBlog;
