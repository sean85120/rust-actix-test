import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { announcementsApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminAnnouncements = () => {
  const { t, i18n } = useTranslation();
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
        setMessage({ type: 'success', text: t('admin.announcementCreated') });
      } else {
        await announcementsApi.update(selectedAnnouncement.id, selectedAnnouncement);
        setMessage({ type: 'success', text: t('admin.announcementUpdated') });
      }
      setIsModalOpen(false);
      fetchAnnouncements();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.operationFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handlePublish = async (id) => {
    try {
      await announcementsApi.publish(id);
      setMessage({ type: 'success', text: t('admin.announcementPublished') });
      fetchAnnouncements();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.publishFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleArchive = async (id) => {
    try {
      await announcementsApi.archive(id);
      setMessage({ type: 'success', text: t('admin.announcementArchived') });
      fetchAnnouncements();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.archiveFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm(t('admin.deleteConfirmAnnouncement'))) return;
    try {
      await announcementsApi.delete(id);
      setMessage({ type: 'success', text: t('admin.announcementDeleted') });
      fetchAnnouncements();
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

  const getPriorityLabel = (priority) => {
    return t(`admin.priorities.${priority}`, { defaultValue: priority });
  };

  const getAudienceLabel = (audience) => {
    return t(`admin.audiences.${audience}`, { defaultValue: audience });
  };

  const getStatusLabel = (status) => {
    return t(`admin.statuses.${status}`, { defaultValue: status });
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>{t('admin.announcementsManagement')}</h1>
        <button className="btn-add" onClick={handleCreate}>{t('admin.addAnnouncement')}</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>{t('admin.title')}</th>
              <th>{t('admin.priority')}</th>
              <th>{t('admin.audience')}</th>
              <th>{t('courses.status')}</th>
              <th>{t('admin.published')}</th>
              <th>{t('admin.actions')}</th>
            </tr>
          </thead>
          <tbody>
            {announcements.length === 0 ? (
              <tr>
                <td colSpan="6" className="empty-state">{t('admin.noAnnouncementsFound')}</td>
              </tr>
            ) : (
              announcements.map((announcement) => (
                <tr key={announcement.id}>
                  <td>{announcement.title}</td>
                  <td>
                    <span className={`priority-badge ${announcement.priority}`}>
                      {getPriorityLabel(announcement.priority)}
                    </span>
                  </td>
                  <td>{getAudienceLabel(announcement.target_audience)}</td>
                  <td>
                    <span className={`status-badge ${announcement.status}`}>
                      {getStatusLabel(announcement.status)}
                    </span>
                  </td>
                  <td>{formatDate(announcement.published_at)}</td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(announcement)}>
                        {t('admin.edit')}
                      </button>
                      {announcement.status === 'draft' && (
                        <button className="btn-action publish" onClick={() => handlePublish(announcement.id)}>
                          {t('admin.publish')}
                        </button>
                      )}
                      {announcement.status === 'published' && (
                        <button className="btn-action" onClick={() => handleArchive(announcement.id)}>
                          {t('admin.archive')}
                        </button>
                      )}
                      <button className="btn-action delete" onClick={() => handleDelete(announcement.id)}>
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
        title={isCreating ? t('admin.createAnnouncement') : t('admin.editAnnouncement')}
      >
        {selectedAnnouncement && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>{t('admin.title')}</label>
              <input
                type="text"
                value={selectedAnnouncement.title}
                onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, title: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>{t('admin.content')}</label>
              <textarea
                value={selectedAnnouncement.content}
                onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, content: e.target.value })}
                required
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>{t('admin.priority')}</label>
                <select
                  value={selectedAnnouncement.priority}
                  onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, priority: e.target.value })}
                >
                  {priorities.map((priority) => (
                    <option key={priority} value={priority}>
                      {getPriorityLabel(priority)}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label>{t('admin.targetAudience')}</label>
                <select
                  value={selectedAnnouncement.target_audience}
                  onChange={(e) => setSelectedAnnouncement({ ...selectedAnnouncement, target_audience: e.target.value })}
                >
                  {targetAudiences.map((audience) => (
                    <option key={audience} value={audience}>
                      {getAudienceLabel(audience)}
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

export default AdminAnnouncements;
