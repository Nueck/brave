// import { url } from 'inspector';

/** 请求服务的环境配置 */
type ServiceEnv = Record<ServiceEnvType, ServiceEnvConfig>;

/** 不同请求服务的环境配置 */
const serviceEnv: ServiceEnv = {
  dev: {
    url: 'http://127.0.0.1:2078/api',
    urlPattern: '/api'
  },
  test: {
    url: 'http://127.0.0.1:2078/api',
    urlPattern: '/api'
  },
  prod: {
    url: 'http://127.0.0.1:2078/api',
    urlPattern: '/api'
  }
};

/**
 * 获取当前环境模式下的请求服务的配置
 * @param env 环境
 */
export function getServiceEnvConfig(env: ImportMetaEnv) {
  const { VITE_SERVICE_ENV = 'dev' } = env;

  const config = serviceEnv[VITE_SERVICE_ENV];

  return config;
}
