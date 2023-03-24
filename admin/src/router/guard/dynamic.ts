import type { NavigationGuardNext, RouteLocationNormalized } from 'vue-router';
import { storeToRefs } from 'pinia';
import { routeName } from '@/router';
import { useInitStore, useRouteStore } from '@/store';
import { localStg } from '@/utils';

/**
 * 动态路由
 */
export async function createDynamicRouteGuard(
  to: RouteLocationNormalized,
  _from: RouteLocationNormalized,
  next: NavigationGuardNext
) {
  const route = useRouteStore();
  const isLogin = Boolean(localStg.get('token'));
  const init = useInitStore();
  const { initStatus } = storeToRefs(init);

  // 初始化权限路由
  if (!route.isInitAuthRoute) {
    /* 获取初始化状态 */

    if (!initStatus) {
      /* 未初始化的时候添加系统初始化路由 */
      route.addInitRoute();
    } else {
      /* 初始化成功的时候删除系统初始化路由 */
      route.removeInitRoutes();
    }

    // 未初始化跳到初始化界面
    if (!initStatus) {
      // 未初始化情况下直接回到初始化页
      next({ name: routeName('init') });
    } else if (!isLogin) {
      const toName = to.name as AuthRoute.AllRouteKey;
      if (route.isValidConstantRoute(toName) && !to.meta.requiresAuth) {
        next();
      } else {
        const redirect = to.fullPath;
        next({ name: routeName('login'), query: { redirect } });
      }
      return false;
    }

    await route.initAuthRoute();

    if (to.name === routeName('not-found')) {
      // 动态路由没有加载导致被not-found路由捕获，等待权限路由加载好了，回到之前的路由
      // 若路由是从根路由重定向过来的，重新回到根路由
      const ROOT_ROUTE_NAME: AuthRoute.AllRouteKey = 'root';
      const path = to.redirectedFrom?.name === ROOT_ROUTE_NAME ? '/' : to.fullPath;
      next({ path, replace: true, query: to.query, hash: to.hash });
      return false;
    }
  }

  /* 初始化后路由后删除系统路由 */
  if (initStatus) {
    route.removeInitRoutes();
  }

  // 权限路由已经加载，仍然未找到，重定向到404
  if (to.name === routeName('not-found')) {
    next({ name: routeName('404'), replace: true });
    return false;
  }

  return true;
}
