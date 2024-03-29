import { unref, nextTick } from 'vue';
import { defineStore } from 'pinia';
import { router } from '@/router';
import { fetchLogin, fetchUserInfo, fetchEmailLogin, fetchUserDataInfo, fetchRegister } from '@/service';
import { useRouterPush } from '@/composables';
import { localStg } from '@/utils';
import { toLoginModule } from '~/src/views/_builtin/login/components';
import { useTabStore } from '../tab';
import { useRouteStore } from '../route';
import { getToken, getUserInfo, clearAuthStorage, getTempInfo, setTempInfo, removeTempInfo } from './helpers';

interface AuthState {
  /** 用户信息 */
  userInfo: Auth.UserInfo;
  /** 用户token */
  token: string | null;
  /** 登录的加载状态 */
  loginLoading: boolean;
  // 用户的临时信息
  tempInfo: Temp.TempInfo | null;

  registerLoading: boolean;
}

export const useAuthStore = defineStore('auth-store', {
  state: (): AuthState => ({
    userInfo: getUserInfo(),
    token: getToken(),
    loginLoading: false,
    registerLoading: false,
    tempInfo: getTempInfo()
  }),
  getters: {
    /** 是否登录 */
    isLogin(state) {
      return Boolean(state.token);
    }
  },
  actions: {
    // 设置临时的信息
    setTempInfoToLocal(name: string, pwd: string) {
      setTempInfo(name, pwd);
    },
    // 删除临时信息
    removeTempInfoFormLocal() {
      removeTempInfo();
    },

    /* 获取用户数据信息 */
    async getUserDataInfo() {
      const { data } = await fetchUserDataInfo();
      if (data) {
        return data;
      }
      const emptyInfo: ApiAuth.UserDataInfo = {
        visitCount: 0,
        readCount: 0,
        messagesCount: 0,
        articleNum: 0
      };
      return emptyInfo;
    },

    /** 重置auth状态 */
    resetAuthStore() {
      const { toLogin } = useRouterPush(false);
      const { resetTabStore } = useTabStore();
      const { resetRouteStore } = useRouteStore();
      const route = unref(router.currentRoute);

      clearAuthStorage();
      this.$reset();

      if (route.meta.requiresAuth) {
        toLogin();
      }

      nextTick(() => {
        resetTabStore();
        resetRouteStore();
      });
    },

    async handleActionAfterLogin(backendToken: ApiAuth.Token) {
      const route = useRouteStore();
      const { toLoginRedirect } = useRouterPush(false);

      const loginSuccess = await this.handleUserToken(backendToken);

      if (loginSuccess) {
        await route.initAuthRoute();

        // 跳转登录后的地址
        toLoginRedirect();

        // 登录成功弹出欢迎提示
        if (route.isInitAuthRoute) {
          window.$notification?.success({
            title: '登录成功!',
            content: `欢迎回来，${this.userInfo.userName}!`,
            duration: 3000
          });
        }

        return;
      }

      // 不成功则重置状态;
      this.resetAuthStore();
    },

    /**
     * 处理返回的token
     */
    async handleUserToken(backendToken: ApiAuth.Token) {
      // 先把token存储到缓存中(后面接口的请求头需要token)
      const { token, refreshToken } = backendToken;
      localStg.set('token', token);
      // 限制三次请求
      const dataToken: StorageInterface.RefreshData = { code: refreshToken, num: 3 };
      localStg.set('refreshToken', dataToken);

      // 获取用户信息
      const { data } = await fetchUserInfo();
      if (data) {
        // 成功后把用户信息存储到缓存中
        localStg.set('userInfo', data);

        // 更新状态
        this.userInfo = data;
        this.token = token;

        return true;
      }

      return false;
    },

    /**
     * 登录
     */
    async login(userName: string, password: string) {
      this.loginLoading = true;

      const { data } = await fetchLogin(userName, password);

      if (data) {
        await this.handleActionAfterLogin(data);
        this.loginLoading = false;
        return true;
      }
      this.loginLoading = false;
      return false;
    },

    /**
     * 邮箱验证码登录
     */
    async email_login(email: string, verify_code: string, code: string) {
      this.loginLoading = true;

      const { data } = await fetchEmailLogin(email, verify_code, code);

      if (data) {
        await this.handleActionAfterLogin(data);
        this.loginLoading = false;
        return true;
      }
      this.loginLoading = false;
      return false;
    },
    /**
     * 注册
     */
    async register(model: any, info: ApiAuth.RegisterInfo) {
      this.registerLoading = true;

      const { error } = await fetchRegister(info);

      if (!error) {
        window.$message?.success('注册成功!');
        this.removeTempInfoFormLocal();
        const { username, pwd } = model;
        this.setTempInfoToLocal(username, pwd);

        setTimeout(() => {
          toLoginModule('pwd-login');
        }, 500);
      }
      this.registerLoading = false;
    }
  }
});
