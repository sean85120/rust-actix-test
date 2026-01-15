import { useState, useEffect } from 'react';
import { membersApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminMembers = () => {
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
      setMessage({ type: 'success', text: 'Member updated successfully!' });
      setIsModalOpen(false);
      fetchMembers();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Update failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm('Are you sure you want to delete this member?')) return;
    try {
      await membersApi.delete(id);
      setMessage({ type: 'success', text: 'Member deleted successfully!' });
      fetchMembers();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Delete failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const formatDate = (dateStr) => {
    return new Date(dateStr).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>Members Management</h1>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Email</th>
              <th>Phone</th>
              <th>Role</th>
              <th>Joined</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {members.length === 0 ? (
              <tr>
                <td colSpan="6" className="empty-state">No members found</td>
              </tr>
            ) : (
              members.map((member) => (
                <tr key={member.id}>
                  <td>{member.first_name} {member.last_name}</td>
                  <td>{member.email}</td>
                  <td>{member.phone || '-'}</td>
                  <td>
                    <span className={`role-badge ${member.role}`}>{member.role}</span>
                  </td>
                  <td>{formatDate(member.created_at)}</td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(member)}>
                        Edit
                      </button>
                      <button className="btn-action delete" onClick={() => handleDelete(member.id)}>
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

      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Edit Member">
        {selectedMember && (
          <form onSubmit={handleUpdate} className="admin-form">
            <div className="form-row">
              <div className="form-group">
                <label>First Name</label>
                <input
                  type="text"
                  value={selectedMember.first_name}
                  onChange={(e) => setSelectedMember({ ...selectedMember, first_name: e.target.value })}
                  required
                />
              </div>
              <div className="form-group">
                <label>Last Name</label>
                <input
                  type="text"
                  value={selectedMember.last_name}
                  onChange={(e) => setSelectedMember({ ...selectedMember, last_name: e.target.value })}
                  required
                />
              </div>
            </div>
            <div className="form-group">
              <label>Phone</label>
              <input
                type="tel"
                value={selectedMember.phone || ''}
                onChange={(e) => setSelectedMember({ ...selectedMember, phone: e.target.value })}
              />
            </div>
            <div className="form-group">
              <label>Role</label>
              <select
                value={selectedMember.role}
                onChange={(e) => setSelectedMember({ ...selectedMember, role: e.target.value })}
              >
                <option value="member">Member</option>
                <option value="instructor">Instructor</option>
                <option value="admin">Admin</option>
              </select>
            </div>
            <div className="form-actions">
              <button type="button" className="btn-cancel-form" onClick={() => setIsModalOpen(false)}>
                Cancel
              </button>
              <button type="submit" className="btn-save">Save Changes</button>
            </div>
          </form>
        )}
      </Modal>
    </div>
  );
};

export default AdminMembers;
