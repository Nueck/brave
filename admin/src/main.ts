import { createApp } from 'vue';
import App from './App.vue';
import AppLoading from './components/common/AppLoading.vue';
import { setupDirectives } from './directives';
import { setupRouter } from './router';
import { setupAssets } from './plugins';
import { setupStore, useInitStore } from './store';
import { setupI18n } from './locales';

async function setupApp() {
  // import assets: js、css
  setupAssets();
  // app loading
  const appLoading = createApp(AppLoading);

  appLoading.mount('#appLoading');

  const app = createApp(App);

  // store plugin: pinia
  setupStore(app);

  // vue custom directives
  setupDirectives(app);

  // vue router
  await setupRouter(app);

  setupI18n(app);

  /* 处理初始化状态 */
  /* 初始化系统配置 */
  const { initStatusStore } = useInitStore();

  await initStatusStore();

  // mount app
  app.mount('#app');
}

setupApp();
