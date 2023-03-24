/** 用户角色 */
export enum EnumUserRole {
  super = '超级管理员',
  admin = '管理员',
  user = '普通用户'
}

/** 登录模块 */
export enum EnumLoginModule {
  'pwd-login' = '账密登录',
  'code-login' = '验证码登录',
  'register' = '注册',
  'reset-pwd' = '重置密码'
}

/** 初始化模块 */
export enum EnumInitModule {
  'user' = '用户配置'
}
