/** 用户相关模块 */
declare namespace Auth {
  /**
   * 用户角色类型(前端静态路由用角色类型进行路由权限的控制)
   * - super: 超级管理员(该权限具有所有路由数据)
   * - admin: 管理员
   * - user: 用户
   * - custom: 自定义角色
   */
  type RoleType = keyof typeof import('@/enum').EnumUserRole;

  /** 用户信息 */
  interface UserInfo {
    /** 用户id */
    userId: string;
    /** 用户名 */
    userName: string;
    /** 用户角色类型 */
    userRole: RoleType;
    /** 用户的blog地址 */
    userHomeUrl: string;
  }

  interface UserDataInfo {
    /** 访问量 */
    visitCount: number;
    /** 阅读量 */
    readCount: number;
    /** 留言量 */
    messagesCount: number;
    /** 文章数 */
    articleNum: number;
  }

  /* 注册信息 */

  interface RegisterInfo {
    /** 用户名 */
    username: string;
    /** 用户邮箱 */
    email: string;
    /** 用户密码 */
    password: string;
    /** 验证码 */
    verify_code: string;
    /** token */
    code: string;
  }

  /* 忘记密码 */
  interface ForgetInfo {
    /** 用户邮箱 */
    email: string;
    /** 用户新密码 */
    new_pwd: string;
    /** 验证码 */
    verify_code: string;
    /** token */
    code: string;
  }
}

// 用户的临时信息
declare namespace Temp {
  interface TempInfo {
    /** 用户名 */
    userName: string;
    /** 用户密码 */
    userPwd: string;
  }
}

declare namespace UserManagement {
  interface User extends ApiUserManagement.User {
    /** 序号 */
    index: number;
    /** 表格的key（id） */
    key: number;
  }

  /**
   * 用户状态
   * - 1: 启用
   * - 2: 禁用
   * - 3: 冻结
   * - 4: 软删除
   */
  type UserStatusKey = NonNullable<User['user_status']>;

  // 用户权限
  type UserAuthority = NonNullable<User['authority']>;
}
