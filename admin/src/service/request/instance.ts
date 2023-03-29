import axios from 'axios';
import type { AxiosError, AxiosInstance, AxiosRequestConfig } from 'axios';
import { useAuthStore } from '@/store';
import {
  localStg,
  handleAxiosError,
  handleBackendError,
  handleResponseError,
  handleServiceResult,
  transformRequestData
} from '@/utils';
import { handleRefreshToken } from './helpers';

/**
 * 封装axios请求类
 */
export default class CustomAxiosInstance {
  instance: AxiosInstance;

  backendConfig: Service.BackendResultConfig;

  /**
   *
   * @param axiosConfig - axios配置
   * @param backendConfig - 后端返回的数据配置
   */
  constructor(
    axiosConfig: AxiosRequestConfig,
    backendConfig: Service.BackendResultConfig = {
      state: 'state',
      data: 'data',
      message: 'message'
    }
  ) {
    this.backendConfig = backendConfig;
    this.instance = axios.create(axiosConfig);
    this.setInterceptor();
  }

  /** 设置请求拦截器 */
  setInterceptor() {
    this.instance.interceptors.request.use(
      async config => {
        const handleConfig = { ...config };
        if (handleConfig.headers) {
          // 数据转换
          const contentType = handleConfig.headers['Content-Type'] as string;
          handleConfig.data = await transformRequestData(handleConfig.data, contentType);
          // 设置 Bearer token
          if (!handleConfig.headers.Authorization)
            handleConfig.headers.Authorization = `Bearer ${localStg.get('token') || ''}`;
        }
        return handleConfig;
      },
      (axiosError: AxiosError) => {
        const error = handleAxiosError(axiosError);
        return handleServiceResult(error, null, null);
      }
    );
    this.instance.interceptors.response.use(
      async response => {
        const { status } = response;

        if (status === 200) {
          const backend = response.data;
          const { state, data, message } = this.backendConfig;
          // 请求成功
          if (backend[state] === 'success') {
            return handleServiceResult(null, backend[data], backend[message]);
          } else if (backend[state] === 'error') {
            /* 显示弹窗 */
            const error = handleBackendError(backend, this.backendConfig);
            return handleServiceResult(error, null, backend[message]);
          }
        }
        const error = handleResponseError(response);
        return handleServiceResult(error, null, null);
      },
      async (axiosError: AxiosError) => {
        // 如何未授权更新接口
        if (axiosError.response?.status === 401) {
          /* 如果未授权尝试重新请求token */
          const config = await handleRefreshToken(axiosError.config);
          if (config) {
            return this.instance.request(config);
          }
          useAuthStore().resetAuthStore();
        }

        const error = handleAxiosError(axiosError);
        return handleServiceResult(error, null, null);
      }
    );
  }
}
