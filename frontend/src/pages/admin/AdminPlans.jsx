import { useState, useEffect } from 'react';
import { plansApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminPlans = () => {
  const [plans, setPlans] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedPlan, setSelectedPlan] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [message, setMessage] = useState({ type: '', text: '' });

  const emptyPlan = {
    name: '',
    description: '',
    duration_days: 30,
    price: 0,
    max_bookings_per_month: 10,
    is_active: true,
  };

  useEffect(() => {
    fetchPlans();
  }, []);

  const fetchPlans = async () => {
    try {
      setLoading(true);
      const response = await plansApi.list();
      setPlans(response.data.plans || []);
    } catch (error) {
      console.error('Error fetching plans:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => {
    setSelectedPlan(emptyPlan);
    setIsCreating(true);
    setIsModalOpen(true);
  };

  const handleEdit = (plan) => {
    setSelectedPlan(plan);
    setIsCreating(false);
    setIsModalOpen(true);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isCreating) {
        await plansApi.create(selectedPlan);
        setMessage({ type: 'success', text: 'Plan created successfully!' });
      } else {
        await plansApi.update(selectedPlan.id, selectedPlan);
        setMessage({ type: 'success', text: 'Plan updated successfully!' });
      }
      setIsModalOpen(false);
      fetchPlans();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Operation failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm('Are you sure you want to delete this plan?')) return;
    try {
      await plansApi.delete(id);
      setMessage({ type: 'success', text: 'Plan deleted!' });
      fetchPlans();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || 'Delete failed' });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const formatPrice = (price) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(price);
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>Membership Plans Management</h1>
        <button className="btn-add" onClick={handleCreate}>+ Add Plan</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Duration</th>
              <th>Price</th>
              <th>Bookings/Month</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {plans.length === 0 ? (
              <tr>
                <td colSpan="6" className="empty-state">No plans found</td>
              </tr>
            ) : (
              plans.map((plan) => (
                <tr key={plan.id}>
                  <td>
                    <div>
                      <strong>{plan.name}</strong>
                      <br />
                      <small style={{ color: '#888' }}>{plan.description}</small>
                    </div>
                  </td>
                  <td>{plan.duration_days} days</td>
                  <td>{formatPrice(plan.price)}</td>
                  <td>{plan.max_bookings_per_month || 'Unlimited'}</td>
                  <td>
                    <span className={`status-badge ${plan.is_active ? 'active' : 'inactive'}`}>
                      {plan.is_active ? 'Active' : 'Inactive'}
                    </span>
                  </td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(plan)}>
                        Edit
                      </button>
                      <button className="btn-action delete" onClick={() => handleDelete(plan.id)}>
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
        title={isCreating ? 'Create Plan' : 'Edit Plan'}
      >
        {selectedPlan && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>Plan Name</label>
              <input
                type="text"
                value={selectedPlan.name}
                onChange={(e) => setSelectedPlan({ ...selectedPlan, name: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>Description</label>
              <textarea
                value={selectedPlan.description}
                onChange={(e) => setSelectedPlan({ ...selectedPlan, description: e.target.value })}
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>Duration (days)</label>
                <input
                  type="number"
                  value={selectedPlan.duration_days}
                  onChange={(e) => setSelectedPlan({ ...selectedPlan, duration_days: parseInt(e.target.value) })}
                  min="1"
                  required
                />
              </div>
              <div className="form-group">
                <label>Price ($)</label>
                <input
                  type="number"
                  step="0.01"
                  value={selectedPlan.price}
                  onChange={(e) => setSelectedPlan({ ...selectedPlan, price: parseFloat(e.target.value) })}
                  min="0"
                  required
                />
              </div>
            </div>
            <div className="form-group">
              <label>Max Bookings per Month (0 for unlimited)</label>
              <input
                type="number"
                value={selectedPlan.max_bookings_per_month || 0}
                onChange={(e) => setSelectedPlan({ ...selectedPlan, max_bookings_per_month: parseInt(e.target.value) || null })}
                min="0"
              />
            </div>
            <div className="form-group checkbox-group">
              <input
                type="checkbox"
                id="is_active"
                checked={selectedPlan.is_active}
                onChange={(e) => setSelectedPlan({ ...selectedPlan, is_active: e.target.checked })}
              />
              <label htmlFor="is_active">Active</label>
            </div>
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

export default AdminPlans;
