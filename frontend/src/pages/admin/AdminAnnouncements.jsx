import { useState, useEffect } from 'react';
import { announcementsApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminAnnouncements = () => {
  const [announcements, setAnnouncements] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedAnnouncement, setSelectedAnnouncement] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [message, setMessage] = useState({ type: '', text: '' });

  const priorities = ['low', 'normal', 'high', 'urgent'];
  const targetAudiences = ['all', 'members', 'instructors', 'staff'];

  const emptyAnnouncement = {
    title: '',
    content: '',
    priority: 'normal',
    target_audience: 'all',
    publish_immediately: false,
  };

  useEffect(() => {
    fetchAnnouncements();
  }, []);

  const fetchAnnouncements = async () => {
    try {
      setLoading(true);
      const response = await announcementsApi.list();
      setAnnouncements(response.data.announcements || []);
    } catch (error) {
      console.error('Error fetching announcements:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setSelectedAnnouncement(emptyAnnouncement);
    setIsCreating(true);
    setIsModalOpen(true);
  };

  const handleEdit = (announcement) => {
    setSelectedAnnouncement(announcement);
    setIsCreating(false);
    setIsModalOpen(true);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isCreating) {
        await announcementsApi.create(selectedAnnouncement);
        setMessage({ type: 'success', text: 'Announcement created successfully!' });
      } else {
        await announcementsApi.update(selectedAnnouncement.id, selectedAnnouncement);
        setMessage({ type: 'success', text: 'Announcement updated successfully!' });
      }
      setIsModalOpen(false);
      fetchAnnouncements();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Operation failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handlePublish = async (id) => {
    try {
      await announcementsApi.publish(id);
      setMessage({ type: 'success', text: 'Announcement published!' });
      fetchAnnouncements();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Publish failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleArchive = async (id) => {
    try {
      await announcementsApi.archive(id);
      setMessage({ type: 'success', text: 'Announcement archived!' });
      fetchAnnouncements();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Archive failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm('Are you sure you want to delete this announcement?')) return;
    try {
      await announcementsApi.delete(id);
      setMessage({ type: 'success', text: 'Announcement deleted!' });
      fetchAnnouncements();
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
        <h1>Announcements Management</h1>
        <button className="btn-add" onClick={handleCreate}>+ Add Announcement</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>Title</th>
              <th>Priority</th>
              <th>Audience</th>
              <th>Status</th>
              <th>Published</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {announcements.length === 0 ? (
              <tr>
                <td colSpan="6" className="empty-state">No announcements found</td>
              </tr>
            ) : (
              announcements.map((announcement) => (
                <tr key={announcement.id}>
                  <td>{announcement.title}</td>
                  <td>
                    <span className={`priority-badge ${announcement.priority}`}>
                      {announcement.priority}
                    </span>
                  </td>
                  <td>{announcement.target_audience}</td>
                  <td>
                    <span className={`status-badge ${announcement.status}`}>
                      {announcement.status}
                    </span>
                  </td>
                  <td>{formatDate(announcement.published_at)}</td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(announcement)}>
                        Edit
                      </button>
                      {announcement.status === 'draft' && (
                        <button className="btn-action publish" onClick={() => handlePublish(announcement.id)}>
                          Publish
                        </button>
                      )}
                      {announcement.status === 'published' && (
                        <button className="btn-action" onClick={() => handleArchive(announcement.id)}>
                          Archive
                        </button>
                      )}
                      <button className="btn-action delete" onClick={() => handleDelete(announcement.id)}>
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
        title={isCreating ? 'Create Announcement' : 'Edit Announcement'}
      >
        {selectedAnnouncement && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>Title</label>
              <input
                type="text"
                value={selectedAnnouncement.title}
                onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, title: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>Content</label>
              <textarea
                value={selectedAnnouncement.content}
                onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, content: e.target.value })}
                required
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>Priority</label>
                <select
                  value={selectedAnnouncement.priority}
                  onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, priority: e.target.value })}
                >
                  {priorities.map((priority) => (
                    <option key={priority} value={priority}>
                      {priority.charAt(0).toUpperCase() + priority.slice(1)}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label>Target Audience</label>
                <select
                  value={selectedAnnouncement.target_audience}
                  onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, target_audience: e.target.value })}
                >
                  {targetAudiences.map((audience) => (
                    <option key={audience} value={audience}>
                      {audience.charAt(0).toUpperCase() + audience.slice(1)}
                    </option>
                  ))}
                </select>
              </div>
            </div>
            {isCreating && (
              <div className="form-group checkbox-group">
                <input
                  type="checkbox"
                  id="publish_immediately"
                  checked={selectedAnnouncement.publish_immediately}
                  onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, publish_immediately: e.target.checked })}
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

export default AdminAnnouncements;
