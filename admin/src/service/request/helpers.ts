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
  const refreshToken = localStg.get('refreshToken') || '';

  /* 将本地token设置成ref */
  const oldConfig = { ...axiosConfig };
  if (oldConfig.headers) {
    oldConfig.headers.Authorization = refreshToken;
  }

  const { data } = await fetchUpdateToken(refreshToken, oldConfig);

  if (data) {
    localStg.set('token', data.token);
    localStg.set('refreshToken', data.refreshToken);

    const config = { ...axiosConfig };
    if (config.headers) {
      config.headers.Authorization = data.token;
    }
    return config;
  }

  resetAuthStore();
  return null;
}
