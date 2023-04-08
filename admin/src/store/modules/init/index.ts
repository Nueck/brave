import { defineStore } from 'pinia';
import { fetchInitStatus } from '~/src/service/api';

interface InitState {
  /** 是否初始化 */
  initStatus: boolean;
}

export const useInitStore = defineStore('init-store', {
  state: (): InitState => ({
    initStatus: true
  }),
  actions: {
    async initStatusStore() {
      const { error } = await fetchInitStatus();

      if (error) {
        this.initStatus = false;
      } else {
        this.initStatus = true;
      }
    }
  }
});
