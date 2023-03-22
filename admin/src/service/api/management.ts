import { mockRequest } from '../request';

/** 获取用户列表 */
/* 需要写个适配器将数据处理成序号加key的形式 */
export const fetchUserList = async () => {
  return mockRequest.post<UserManagement.User[] | null>('/users');
};
