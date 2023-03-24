import type { App } from 'vue';
import { createPinia } from 'pinia';
import { resetSetupStore } from './plugins';
import { useInitStore } from './modules/init';

/** setup vue store plugin: pinia. - [安装vue状态管理插件：pinia] */
export async function setupStore(app: App) {
  const store = createPinia();
  store.use(resetSetupStore);

  app.use(store);

  /* 初始化系统配置 */
  const { initStatusStore } = useInitStore();

  await initStatusStore();
}

export * from './modules';
export * from './subscribe';
