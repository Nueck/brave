declare namespace Blog {
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

  interface UpdateArticleEditData {
    title: string;
    subtitle: string;
    table_id: number;
    img_url: string;
    content: string;
    html_content: string;
  }
}
