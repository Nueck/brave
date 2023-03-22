const system: AuthRoute.Route = {
  name: 'system',
  path: '/system',
  component: 'basic',
  meta: {
    title: '系统管理',
    icon: 'ant-design:setting-outlined',
    order: 100,
    requiresAuth: true,
    permissions: ['super', 'admin']
  },
  children: [
    {
      name: 'system_dashboard',
      path: '/system/dashboard',
      component: 'self',
      meta: {
        title: '仪表盘',
        icon: 'ri:dashboard-2-line',
        requiresAuth: true,
        permissions: ['super', 'admin']
      }
    },
    {
      name: 'system_user',
      path: '/system/user',
      component: 'self',
      meta: {
        title: '用户管理',
        icon: 'ic:round-supervised-user-circle',
        requiresAuth: true,
        permissions: ['super']
      }
    },
    {
      name: 'system_skin',
      path: '/system/skin',
      component: 'self',
      meta: {
        title: '皮肤管理',
        icon: 'ant-design:skin-outlined',
        requiresAuth: true,
        permissions: ['super']
      }
    }
  ]
};

export default system;
