import type { AxiosRequestConfig } from 'axios';
import { useAuthStore } from '@/store';
import { localStg } from '@/utils';
import { fetchUpdateToken } from '../api';

/**
 * 刷新token
 * @param axiosConfig - token失效时的请求配置
 */
export async function handleRefreshToken(axiosConfig: AxiosRequestConfig) {
  const { resetAuthStore } = useAuthStore();
  const tokenData: StorageInterface.RefreshData = localStg.get('refreshToken') || { code: '', num: 0 };

  if (tokenData.num > 0) {
    /* 将本地token设置成ref */
    const oldConfig = { ...axiosConfig };
    if (oldConfig.headers) {
      oldConfig.headers.Authorization = tokenData.code;
    }

    const { data } = await fetchUpdateToken(oldConfig);

    if (data) {
      localStg.set('token', data.token);
      const newFreshTokenData: StorageInterface.RefreshData = { code: data.refreshToken, num: 3 };
      localStg.set('refreshToken', newFreshTokenData);

      const config = { ...axiosConfig };
      if (config.headers) {
        config.headers.Authorization = data.token;
      }
      return config;
    }

    const newFreshTokenData: StorageInterface.RefreshData = { code: tokenData.code, num: tokenData.num - 1 };
    localStg.set('refreshToken', newFreshTokenData);
  }

  resetAuthStore();
  return null;
}
