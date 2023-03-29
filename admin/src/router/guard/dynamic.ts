import type { NavigationGuardNext, RouteLocationNormalized } from 'vue-router';
import { storeToRefs } from 'pinia';
import { routeName } from '@/router';
import { useInitStore, useRouteStore } from '@/store';
import { localStg } from '@/utils';
import { fetchTokenValid } from '~/src/service';

/**
 * 动态路由
 */
export async function createDynamicRouteGuard(
  to: RouteLocationNormalized,
  _from: RouteLocationNormalized,
  next: NavigationGuardNext
) {
  const route = useRouteStore();

  let isLogin = false;

  // 验证token
  if (localStg.get('token')) {
    const { error } = await fetchTokenValid();
    if (error) {
      isLogin = false;
    } else {
      isLogin = true;
    }
  } else {
    isLogin = false;
  }

  const init = useInitStore();
  const { initStatus } = storeToRefs(init);

  // 初始化权限路由
  if (!route.isInitAuthRoute) {
    /* 根据是否初始化设置路由 */
    if (initStatus.value) {
      route.removeInitRoutes();
    } else {
      route.addInitRoute();
    }

    // 未初始化跳到初始化界面
    if (!initStatus.value) {
      // 未初始化情况下直接回到初始化页~
      const toName = to.name as AuthRoute.AllRouteKey;
      if ((route.isValidConstantRoute(toName) || route.isValidInitRoute(toName)) && !to.meta.requiresAuth) {
        next();
      } else {
        next({ name: routeName('init') });
      }
      return false;
    } else if (!isLogin) {
      const toName = to.name as AuthRoute.AllRouteKey;
      if (route.isValidConstantRoute(toName) && !to.meta.requiresAuth) {
        next();
      } else {
        next({ name: routeName('login') });
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
  if (initStatus.value) {
    route.removeInitRoutes();
  }

  // 权限路由已经加载，仍然未找到，重定向到404
  if (to.name === routeName('not-found')) {
    next({ name: routeName('404'), replace: true });
    return false;
  }

  return true;
}
