/** 用户状态 */
export const userStatusLabels: Record<UserManagement.UserStatusKey, string> = {
  1: '启用',
  2: '禁用',
  3: '冻结',
  4: '软删除'
};

/** 用户状态 */
export const userAuthority: Record<UserManagement.UserAuthority, string> = {
  admin: '管理员',
  super: '超级管理员',
  user: '用户'
};

export const userAuthorityOptions: { value: UserManagement.UserAuthority; label: string }[] = [
  { value: 'admin', label: userAuthority.admin },
  { value: 'super', label: userAuthority.super },
  { value: 'user', label: userAuthority.user }
];

export const userStatusOptions: { value: UserManagement.UserStatusKey; label: string }[] = [
  { value: '1', label: userStatusLabels['1'] },
  { value: '2', label: userStatusLabels['2'] },
  { value: '3', label: userStatusLabels['3'] },
  { value: '4', label: userStatusLabels['4'] }
];
