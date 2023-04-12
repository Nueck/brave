declare namespace StorageInterface {
  /** localStorage的存储数据的类型 */
  interface Session {
    demoKey: string;
  }

  // 刷新token样式
  interface RefreshData {
    code: string;
    num: number;
  }

  /** localStorage的存储数据的类型 */
  interface Local {
    /** 主题颜色 */
    themeColor: string;
    /** 用户token */
    token: string;
    /** 用户刷新token */
    refreshToken: RefreshData;
    /** 用户信息 */
    userInfo: Auth.UserInfo;
    // 用户的临时信息
    tempInfo: Temp.TempInfo;
    /** 主题配置 */
    themeSettings: Theme.Setting;
    /** 多页签路由信息 */
    multiTabRoutes: App.GlobalTabRoute[];
  }
}
