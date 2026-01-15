import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { blogApi } from '../api/client';
import Loading from '../components/Loading';
import './Blog.css';

const Blog = () => {
  const [posts, setPosts] = useState([]);
  const [featuredPosts, setFeaturedPosts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [categoryFilter, setCategoryFilter] = useState('');

  const categories = ['news', 'training', 'nutrition', 'events', 'tips', 'stories'];

  useEffect(() => {
    fetchPosts();
  }, [categoryFilter]);

  const fetchPosts = async () => {
    try {
      setLoading(true);
      const params = {};
      if (categoryFilter) params.category = categoryFilter;

      const [postsRes, featuredRes] = await Promise.all([
        blogApi.list(params),
        categoryFilter ? Promise.resolve({ data: { posts: [] } }) : blogApi.getFeatured(),
      ]);

      setPosts(postsRes.data.posts || []);
      setFeaturedPosts(featuredRes.data.posts || []);
    } catch (error) {
      console.error('Error fetching posts:', error);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateStr) => {
    return new Date(dateStr).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  return (
    <div className="blog-page">
      <div className="page-header">
        <h1>Blog</h1>
        <p>Tips, news, and stories from our boxing community</p>
      </div>

      {/* Featured Posts */}
      {!categoryFilter && featuredPosts.length > 0 && (
        <section className="featured-section">
          <h2>Featured</h2>
          <div className="featured-grid">
            {featuredPosts.map((post) => (
              <Link to={`/blog/${post.slug}`} key={post.id} className="featured-card">
                <span className="featured-badge">Featured</span>
                <span className="post-category">{post.category}</span>
                <h3>{post.title}</h3>
                <p>{post.excerpt}</p>
                <div className="post-meta">
                  <span className="author">{post.author_name}</span>
                  <span className="date">{formatDate(post.published_at || post.created_at)}</span>
                </div>
              </Link>
            ))}
          </div>
        </section>
      )}

      {/* Category Filter */}
      <div className="category-filters">
        <button
          className={`category-btn ${categoryFilter === '' ? 'active' : ''}`}
          onClick={() => setCategoryFilter('')}
        >
          All
        </button>
        {categories.map((category) => (
          <button
            key={category}
            className={`category-btn ${categoryFilter === category ? 'active' : ''}`}
            onClick={() => setCategoryFilter(category)}
          >
            {category.charAt(0).toUpperCase() + category.slice(1)}
          </button>
        ))}
      </div>

      {/* Posts List */}
      {loading ? (
        <Loading />
      ) : posts.length === 0 ? (
        <div className="no-posts">
          <p>No posts found.</p>
        </div>
      ) : (
        <div className="posts-grid">
          {posts.map((post) => (
            <article key={post.id} className="post-card">
              <span className="post-category">{post.category}</span>
              <Link to={`/blog/${post.slug}`} className="post-title">
                <h3>{post.title}</h3>
              </Link>
              <p className="post-excerpt">{post.excerpt}</p>
              {post.tags && post.tags.length > 0 && (
                <div className="post-tags">
                  {post.tags.slice(0, 3).map((tag, index) => (
                    <span key={index} className="tag">#{tag}</span>
                  ))}
                </div>
              )}
              <div className="post-footer">
                <span className="author">{post.author_name}</span>
                <span className="date">{formatDate(post.published_at || post.created_at)}</span>
                <span className="views">{post.view_count} views</span>
              </div>
            </article>
          ))}
        </div>
      )}
    </div>
  );
};

export default Blog;
