declare namespace Blog {
  /* 文章卡片的信息 */
  type ArticlesInfo = Info.ArticlesInfo;
  type ArticlesEditInfo = Info.ArticlesEditInfo;
}

declare namespace Info {
  /* 接受的文章table信息 */
  interface ArticlesInfo {
    table_id: number;
    img_url: string;
    title: string;
  }

  interface ArticlesEditInfo {
    title: string;
    subtitle: string;
    img_url: string;
    content: string;
  }
}
