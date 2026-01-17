import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { plansApi } from '../../api/client';
import Loading from '../../components/Loading';
import Modal from '../../components/Modal';
import '../Admin.css';

const AdminPlans = () => {
  const { t, i18n } = useTranslation();
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
        setMessage({ type: 'success', text: t('admin.planCreated') });
      } else {
        await plansApi.update(selectedPlan.id, selectedPlan);
        setMessage({ type: 'success', text: t('admin.planUpdated') });
      }
      setIsModalOpen(false);
      fetchPlans();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.operationFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const handleDelete = async (id) => {
    if (!confirm(t('admin.deleteConfirmPlan'))) return;
    try {
      await plansApi.delete(id);
      setMessage({ type: 'success', text: t('admin.planDeleted') });
      fetchPlans();
    } catch (error) {
      setMessage({ type: 'error', text: error.response?.data?.message || t('admin.deleteFailed') });
    }
    setTimeout(() => setMessage({ type: '', text: '' }), 3000);
  };

  const formatPrice = (price) => {
    const locale = i18n.language === 'zh-TW' ? 'zh-TW' : 'en-US';
    return new Intl.NumberFormat(locale, {
      style: 'currency',
      currency: 'USD',
    }).format(price);
  };

  if (loading) return <Loading />;

  return (
    <div className="admin-page">
      <div className="admin-header">
        <h1>{t('admin.membershipPlansManagement')}</h1>
        <button className="btn-add" onClick={handleCreate}>{t('admin.addPlan')}</button>
      </div>

      {message.text && (
        <div className={`message ${message.type}`}>{message.text}</div>
      )}

      <div className="data-table">
        <table>
          <thead>
            <tr>
              <th>{t('admin.name')}</th>
              <th>{t('courses.duration')}</th>
              <th>{t('admin.price')}</th>
              <th>{t('admin.bookingsPerMonth')}</th>
              <th>{t('courses.status')}</th>
              <th>{t('admin.actions')}</th>
            </tr>
          </thead>
          <tbody>
            {plans.length === 0 ? (
              <tr>
                <td colSpan="6" className="empty-state">{t('admin.noPlansFound')}</td>
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
                  <td>{plan.duration_days} {t('admin.days')}</td>
                  <td>{formatPrice(plan.price)}</td>
                  <td>{plan.max_bookings_per_month || t('admin.unlimited')}</td>
                  <td>
                    <span className={`status-badge ${plan.is_active ? 'active' : 'inactive'}`}>
                      {plan.is_active ? t('courses.active') : t('courses.inactive')}
                    </span>
                  </td>
                  <td>
                    <div className="actions">
                      <button className="btn-action edit" onClick={() => handleEdit(plan)}>
                        {t('admin.edit')}
                      </button>
                      <button className="btn-action delete" onClick={() => handleDelete(plan.id)}>
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
        title={isCreating ? t('admin.createPlan') : t('admin.editPlan')}
      >
        {selectedPlan && (
          <form onSubmit={handleSubmit} className="admin-form">
            <div className="form-group">
              <label>{t('admin.planName')}</label>
              <input
                type="text"
                value={selectedPlan.name}
                onChange={(e) => setSelectedPlan({ ...selectedPlan, name: e.target.value })}
                required
              />
            </div>
            <div className="form-group">
              <label>{t('admin.description')}</label>
              <textarea
                value={selectedPlan.description}
                onChange={(e) => setSelectedPlan({ ...selectedPlan, description: e.target.value })}
              />
            </div>
            <div className="form-row">
              <div className="form-group">
                <label>{t('admin.durationDays')}</label>
                <input
                  type="number"
                  value={selectedPlan.duration_days}
                  onChange={(e) => setSelectedPlan({ ...selectedPlan, duration_days: parseInt(e.target.value) })}
                  min="1"
                  required
                />
              </div>
              <div className="form-group">
                <label>{t('admin.priceLabel')}</label>
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
              <label>{t('admin.maxBookingsPerMonth')}</label>
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
              <label htmlFor="is_active">{t('courses.active')}</label>
            </div>
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

export default AdminPlans;
