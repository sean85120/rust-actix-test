import { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { blogApi } from '../api/client';
import Loading from '../components/Loading';
import './BlogPost.css';

const BlogPost = () => {
  const { t, i18n } = useTranslation();
  const { slug } = useParams();
  const navigate = useNavigate();
  const [post, setPost] = useState(null);
  const [recentPosts, setRecentPosts] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchPost();
  }, [slug]);

  const fetchPost = async () => {
    try {
      setLoading(true);
      const [postRes, recentRes] = await Promise.all([
        blogApi.getBySlug(slug),
        blogApi.getRecent(5),
      ]);
      setPost(postRes.data);
      setRecentPosts(recentRes.data.posts?.filter((p) => p.slug !== slug) || []);
    } catch (error) {
      console.error('Error fetching post:', error);
      if (error.response?.status === 404) {
        navigate('/blog');
      }
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateStr) => {
    const locale = i18n.language === 'zh-TW' ? 'zh-TW' : 'en-US';
    return new Date(dateStr).toLocaleDateString(locale, {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  const getCategoryLabel = (category) => {
    return t(`blog.categories.${category}`, { defaultValue: category });
  };

  if (loading) return <Loading />;
  if (!post) return <div className="error">{t('blog.postNotFound')}</div>;

  return (
    <div className="blog-post-page">
      <div className="post-container">
        <button onClick={() => navigate('/blog')} className="back-btn">
          &larr; {t('blog.backToBlog')}
        </button>

        <article className="post-content">
          <header className="post-header">
            <span className="post-category">{getCategoryLabel(post.category)}</span>
            <h1>{post.title}</h1>
            <div className="post-meta">
              <span className="author">{t('blog.by')} {post.author_name}</span>
              <span className="date">{formatDate(post.published_at || post.created_at)}</span>
              <span className="views">{post.view_count} {t('blog.views')}</span>
            </div>
            {post.tags && post.tags.length > 0 && (
              <div className="post-tags">
                {post.tags.map((tag, index) => (
                  <span key={index} className="tag">#{tag}</span>
                ))}
              </div>
            )}
          </header>

          <div className="post-body">
            {post.content.split('\n').map((paragraph, index) => (
              <p key={index}>{paragraph}</p>
            ))}
          </div>
        </article>

        {recentPosts.length > 0 && (
          <aside className="sidebar">
            <h3>{t('blog.recentPosts')}</h3>
            <div className="recent-posts">
              {recentPosts.slice(0, 4).map((recentPost) => (
                <Link
                  to={`/blog/${recentPost.slug}`}
                  key={recentPost.id}
                  className="recent-post"
                >
                  <span className="recent-category">{getCategoryLabel(recentPost.category)}</span>
                  <h4>{recentPost.title}</h4>
                  <span className="recent-date">
                    {formatDate(recentPost.published_at || recentPost.created_at)}
                  </span>
                </Link>
              ))}
            </div>
          </aside>
        )}
      </div>
    </div>
  );
};

export default BlogPost;
