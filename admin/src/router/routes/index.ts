// import { getLoginModuleRegExp } from '@/utils';

/** 根路由: / */
export const ROOT_ROUTE: AuthRoute.Route = {
  name: 'root',
  path: '/',
  redirect: import.meta.env.VITE_ROUTE_HOME_PATH,
  meta: {
    title: 'Root'
  }
};

/** 固定的路由 */
export const constantRoutes: AuthRoute.Route[] = [
  ROOT_ROUTE,
  {
    name: 'login',
    path: '/login',
    component: 'self',
    meta: {
      title: '登录',
      singleLayout: 'blank'
    }
  },
  {
    name: 'constant-page',
    path: '/constant-page',
    component: 'self',
    meta: {
      title: '固定页面',
      singleLayout: 'blank'
    }
  },
  {
    name: '403',
    path: '/403',
    component: 'self',
    meta: {
      title: '无权限',
      singleLayout: 'blank'
    }
  },
  {
    name: '404',
    path: '/404',
    component: 'self',
    meta: {
      title: '未找到',
      singleLayout: 'blank'
    }
  },
  {
    name: '500',
    path: '/500',
    component: 'self',
    meta: {
      title: '服务器错误',
      singleLayout: 'blank'
    }
  },
  // 匹配无效路径的路由
  {
    name: 'not-found',
    path: '/:pathMatch(.*)*',
    component: 'blank',
    meta: {
      title: '未找到',
      singleLayout: 'blank'
    }
  }
];

/* 初始化路由 */

export const initRoute: AuthRoute.Route[] = [
  {
    name: 'init',
    path: '/init',
    component: 'self',
    meta: {
      title: '初始化',
      singleLayout: 'blank'
    }
  }
];
