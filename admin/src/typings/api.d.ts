// 后端接口返回的数据类型

/** 后端返回的用户权益相关类型 */
declare namespace ApiAuth {
  /** 返回的token和刷新token */
  interface Token {
    token: string;
    refreshToken: string;
  }
  /** 返回的用户信息 */
  type UserInfo = Auth.UserInfo;
  /* 注册时候的信息 */
  type RegisterInfo = Auth.RegisterInfo;

  /* 忘记密码信息 */
  type ForgetInfo = Auth.ForgetInfo;
}

/** 后端返回的路由相关类型 */
declare namespace ApiRoute {
  /** 后端返回的路由数据类型 */
  interface Route {
    /** 动态路由 */
    routes: AuthRoute.Route[];
    /** 路由首页对应的key */
    home: AuthRoute.AllRouteKey;
  }
}

/* 邮件验证的后的返回值 */
declare namespace ApiEmailVerification {
  /* 用于接受发送邮箱的token */
  interface EmailCode {
    code: string;
  }
}

declare namespace ApiUserManagement {
  interface User {
    /** 用户id */
    userId: string;
    /** 用户名 */
    userName: string;

    // 权限
    authority: 'admin' | 'super' | 'user';

    /** 用户手机号码 */
    phone: string;

    /** 用户邮箱 */
    email: string | null;
    /**
     * 用户状态
     * - 1: 启用
     * - 2: 禁用
     * - 3: 冻结
     * - 4: 软删除
     */
    userStatus: '1' | '2' | '3' | '4' | null;
  }
}
