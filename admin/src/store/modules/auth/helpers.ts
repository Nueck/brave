import { localStg } from '@/utils';

/** 获取token */
export function getToken() {
  return localStg.get('token') || null;
}

/** 获取用户信息 */
export function getUserInfo() {
  const emptyInfo: Auth.UserInfo = {
    userId: '',
    userName: '',
    userRole: 'user'
  };
  const userInfo: Auth.UserInfo = localStg.get('userInfo') || emptyInfo;

  return userInfo;
}

/** 获取临时记住用户信息 */
export function getTempInfo() {
  const tempInfo = localStg.get('tempInfo') || null;

  return tempInfo;
}
/** 设置临时记住用户信息 */
export function setTempInfo(name: string, pwd: string) {
  // 设置记住我为30天
  const CACHE_TIME = 60 * 60 * 24 * 30;
  const info: Temp.TempInfo = {
    userName: name,
    userPwd: pwd
  };
  localStg.set('tempInfo', info, CACHE_TIME);
}

/** 删除临时记住用户信息 */
export function removeTempInfo() {
  localStg.remove('tempInfo');
}

/** 去除用户相关缓存 */
export function clearAuthStorage() {
  localStg.remove('token');
  localStg.remove('refreshToken');
  localStg.remove('userInfo');
}
