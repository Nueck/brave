const information: AuthRoute.Route = {
  name: 'information',
  path: '/information',
  component: 'self',
  meta: {
    title: '首页',
    icon: 'mdi:home-circle-outline',
    singleLayout: 'basic',
    order: 0,
    requiresAuth: true
  }
};

export default information;
