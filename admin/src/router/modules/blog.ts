const blog: AuthRoute.Route = {
  name: 'blog',
  path: '/blog',
  component: 'basic',
  meta: {
    title: '博客管理',
    icon: 'mdi:library-outline',
    order: 2,
    requiresAuth: true,
    permissions: ['admin', 'super', 'user']
  },
  children: [
    {
      name: 'blog_article_edit',
      path: '/blog/article/edit',
      component: 'self',
      meta: {
        title: '编辑',
        icon: 'material-symbols:edit-document-outline',
        requiresAuth: true,
        permissions: ['admin', 'super', 'user'],
        hide: true,
        order: 0,
        activeMenu: 'blog_article_table'
      }
    },
    {
      name: 'blog_article_table',
      path: '/blog/article/table',
      component: 'self',
      meta: {
        title: '文章',
        icon: 'ic:outline-library-books',
        order: 1,
        requiresAuth: true,
        permissions: ['admin', 'super', 'user']
      }
    },
    {
      name: 'blog_category',
      path: '/blog/category',
      component: 'self',
      meta: {
        title: '类别',
        icon: 'bx:category',
        order: 2,
        requiresAuth: true,
        permissions: ['admin', 'super', 'user']
      }
    },
    {
      name: 'blog_message',
      path: '/blog/message',
      component: 'self',
      meta: {
        title: '留言',
        icon: 'mdi:message-badge-outline',
        order: 3,
        requiresAuth: true,
        permissions: ['admin', 'super', 'user']
      }
    },
    {
      name: 'blog_album',
      path: '/blog/album',
      component: 'self',
      meta: {
        title: '相册',
        icon: 'bx:photo-album',
        order: 4,
        requiresAuth: true,
        permissions: ['admin', 'super', 'user']
      }
    },
    {
      name: 'blog_skin',
      path: '/blog/skin',
      component: 'self',
      meta: {
        title: '皮肤',
        icon: 'ant-design:skin-outlined',
        order: 5,
        requiresAuth: true,
        permissions: ['admin', 'super', 'user']
      }
    }
  ]
};

export default blog;
