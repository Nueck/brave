import type { App } from 'vue';
import { createPinia, storeToRefs } from 'pinia';
import { resetSetupStore } from './plugins';
import { useInitStore } from './modules';

/** setup vue store plugin: pinia. - [安装vue状态管理插件：pinia] */
export async function setupStore(app: App) {
  const store = createPinia();
  store.use(resetSetupStore);

  app.use(store);

  /* 获取初始化状态 */
  await useInitStore().initStatusStore();
}

export * from './modules';
export * from './subscribe';
