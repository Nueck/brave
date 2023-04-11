import { adapter } from '@/utils';
import { basicRequest } from '../request';
import { adapterOfFetchUserList } from './management.adapter';

/**
 * 获取用户列表
 */
export async function fetchUsersList() {
  const data = await basicRequest.get<ApiUserManagement.User[]>('/users');
  return adapter(adapterOfFetchUserList, data);
}

/**
 * 删除user数据
 */
export function fetchDeleteUser(user_id: number) {
  return basicRequest.delete(`/user/${user_id}`, {});
}

/**
 * 更新user数据
 */
export function fetchUpdateUser(id: number, user: UserManagement.User) {
  return basicRequest.put(`/user/${id}`, user);
}
