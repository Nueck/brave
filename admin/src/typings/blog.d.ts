declare namespace Blog {
  /* 文章卡片的信息 */

  interface CardInfo {
    id: number;
    imgUrl: string;
    width: number;
    height: number;
    text: string;
  }
  type ArticlesInfo = Info.ArticlesInfo;
}

declare namespace Info {
  /* 接受的文章table信息 */
  interface ArticlesInfo {
    img_url: string;
    title: string;
  }
}
