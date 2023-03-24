/** 初始化 */
declare namespace Init {
  /* 需要发送给后端的数据结构 */
  interface InitInfo {
    username: string;
    email: string;
    password: string;
    address: string;
  }
  /* 初始化状态 */
  interface InitStatus {
    isInit: boolean;
  }
}
