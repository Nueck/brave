import { defineStore } from 'pinia';
import { fetchInitStatus } from '~/src/service/api';

interface InitState {
  /** 是否初始化 */
  initStatus: boolean;
}

export const useInitStore = defineStore('init-store', {
  state: (): InitState => ({
    initStatus: false
  }),
  getters: {},
  actions: {
    async initStatusStore() {
      const { data } = await fetchInitStatus();

      if (data?.isInit) {
        this.initStatus = true;
        return true;
      }
      this.initStatus = false;
      return false;
    }
  }
});
