import type { AxiosRequestConfig } from 'axios';
import { mockRequest, basicRequest } from '../request';

/**
 * 登录
 * @param userName - 用户名
 * @param password - 密码
 */
export function fetchLogin(username: string, password: string) {
  return basicRequest.post<ApiAuth.Token>('/login', { username, password });
}

/**
 * 验证token是否有效
 */
export function fetchTokenValid() {
  return basicRequest.get('/tokencheck');
}

/**
 * 邮箱验证码登陆
 */
export function fetchEmailLogin(email: string, verify_code: string, code: string) {
  return basicRequest.post<ApiAuth.Token>('/email-login', { email, verify_code, code });
}

/**
 * 获取用户信息
 */

export function fetchUserInfo() {
  return basicRequest.get<ApiAuth.UserInfo>('/user/info');
}

/**
 * 获取用户数据信息
 */

export function fetchUserDataInfo() {
  return basicRequest.get<ApiAuth.UserDataInfo>('/user/articles/info');
}

/**
 * 发送邮箱
 */

export function fetchSendEmail(email: string) {
  return basicRequest.post<ApiEmailVerification.EmailCode>('/sendmail', { email });
}

/**
 * 注册
 */

export function fetchRegister(data: ApiAuth.RegisterInfo) {
  return basicRequest.post('/register', data);
}

/**
 * 忘记密码和修改密码
 */

export function fetchForgetAndChangPwd(data: ApiAuth.ForgetInfo) {
  return basicRequest.put('/user/password', data);
}

/**
 * 获取用户路由数据
 * @param userId - 用户id //不开启动态加载用不到
 * @description 后端根据用户id查询到对应的角色类型，并将路由筛选出对应角色的路由数据返回前端
 */
export function fetchUserRoutes(userId: string) {
  return mockRequest.post<ApiRoute.Route>('/getUserRoutes', { userId });
}

/**
 * 刷新token
 * @param refreshToken
 */
export function fetchUpdateToken(config: AxiosRequestConfig) {
  return basicRequest.put<ApiAuth.Token>('/updateToken', config);
}
