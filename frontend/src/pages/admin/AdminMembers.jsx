import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { membersApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminMembers = () => {
  const { t, i18n } = useTranslation();
  const [members, setMembers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedMember, setSelectedMember] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [message, setMessage] = useState({ type: '', text: '' });

  useEffect(() => {
    fetchMembers();
  }, []);

  const fetchMembers = async () => {
    try {
      setLoading(true);
      const response = await membersApi.list();
      setMembers(response.data.members || []);
    } catch (error) {
      console.error('Error fetching members:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleEdit = (member) => {
    setSelectedMember(member);
    setIsModalOpen(true);
  };

  const handleUpdate = async (e) => {
    e.preventDefault();
    try {
      await membersApi.update(selectedMember.id, {
        first_name: selectedMember.first_name,
        last_name: selectedMember.last_name,
        phone: selectedMember.phone,
        role: selectedMember.role,
      });
      setMessage({ type: 'success', text: t('admin.memberUpdated') });
      setIsModalOpen(false);
      fetchMembers();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.updateFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm(t('admin.deleteConfirmMember'))) return;
    try {
      await membersApi.delete(id);
      setMessage({ type: 'success', text: t('admin.memberDeleted') });
      fetchMembers();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.deleteFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const formatDate = (dateStr) => {
    const locale = i18n.language === 'zh-TW' ? 'zh-TW' : 'en-US';
    return new Date(dateStr).toLocaleDateString(locale, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  const getRoleLabel = (role) => {
    return t(`admin.roles.${role}`, { defaultValue: role });
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>{t('admin.membersManagement')}</h1>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>{t('admin.name')}</th>
              <th>{t('admin.email')}</th>
              <th>{t('admin.phone')}</th>
              <th>{t('admin.role')}</th>
              <th>{t('admin.joined')}</th>
              <th>{t('admin.actions')}</th>
            </tr>
          </thead>
          <tbody>
            {members.length === 0 ? (
              <tr>
                <td colSpan="6" className="empty-state">{t('admin.noMembersFound')}</td>
              </tr>
            ) : (
              members.map((member) => (
                <tr key={member.id}>
                  <td>{member.first_name} {member.last_name}</td>
                  <td>{member.email}</td>
                  <td>{member.phone || '-'}</td>
                  <td>
                    <span className={`role-badge ${member.role}`}>{getRoleLabel(member.role)}</span>
                  </td>
                  <td>{formatDate(member.created_at)}</td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(member)}>
                        {t('admin.edit')}
                      </button>
                      <button className="btn-action delete" onClick={() => handleDelete(member.id)}>
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

      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title={t('admin.editMember')}>
        {selectedMember && (
          <form onSubmit={handleUpdate} className="admin-form">
            <div className="form-row">
              <div className="form-group">
                <label>{t('auth.firstName')}</label>
                <input
                  type="text"
                  value={selectedMember.first_name}
                  onChange={(e) => setSelectedMember({ ...selectedMember, first_name: e.target.value })}
                  required
                />
              </div>
              <div className="form-group">
                <label>{t('auth.lastName')}</label>
                <input
                  type="text"
                  value={selectedMember.last_name}
                  onChange={(e) => setSelectedMember({ ...selectedMember, last_name: e.target.value })}
                  required
                />
              </div>
            </div>
            <div className="form-group">
              <label>{t('admin.phone')}</label>
              <input
                type="tel"
                value={selectedMember.phone || ''}
                onChange={(e) => setSelectedMember({ ...selectedMember, phone: e.target.value })}
              />
            </div>
            <div className="form-group">
              <label>{t('admin.role')}</label>
              <select
                value={selectedMember.role}
                onChange={(e) => setSelectedMember({ ...selectedMember, role: e.target.value })}
              >
                <option value="member">{t('admin.roles.member')}</option>
                <option value="instructor">{t('admin.roles.instructor')}</option>
                <option value="admin">{t('admin.roles.admin')}</option>
              </select>
            </div>
            <div className="form-actions">
              <button type="button" className="btn-cancel-form" onClick={() => setIsModalOpen(false)}>
                {t('admin.cancel')}
              </button>
              <button type="submit" className="btn-save">{t('admin.saveChanges')}</button>
            </div>
          </form>
        )}
      </Modal>
    </div>
  );
};

export default AdminMembers;
