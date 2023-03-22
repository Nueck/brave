import type { RouteComponent } from 'vue-router';

export const views: Record<
  PageRoute.LastDegreeRouteKey,
  RouteComponent | (() => Promise<{ default: RouteComponent }>)
> = {
  403: () => import('./_builtin/403/index.vue'),
  404: () => import('./_builtin/404/index.vue'),
  500: () => import('./_builtin/500/index.vue'),
  'constant-page': () => import('./_builtin/constant-page/index.vue'),
  login: () => import('./_builtin/login/index.vue'),
  'not-found': () => import('./_builtin/not-found/index.vue'),
  about: () => import('./about/index.vue'),
  blog_album: () => import('./blog/album/index.vue'),
  blog_article_edit: () => import('./blog/article/edit/index.vue'),
  blog_article_table: () => import('./blog/article/table/index.vue'),
  blog_category: () => import('./blog/category/index.vue'),
  blog_message: () => import('./blog/message/index.vue'),
  blog_skin: () => import('./blog/skin/index.vue'),
  information: () => import('./information/index.vue'),
  system_dashboard: () => import('./system/dashboard/index.vue'),
  system_skin: () => import('./system/skin/index.vue'),
  system_user: () => import('./system/user/index.vue')
};
