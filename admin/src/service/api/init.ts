// import type { AxiosRequestConfig } from 'axios';
import { basicRequest } from '../request';

/* 初始化设置 */

/**
 * 初始化
 */
export function fetchInit(info: Init.InitInfo) {
  return basicRequest.post<Init.InitStatus>('/init', info);
}

/**
 * 用于判断系统初始化状态的
 */
export function fetchInitStatus() {
  return basicRequest.get<Init.InitStatus>('/init/state', {});
}
