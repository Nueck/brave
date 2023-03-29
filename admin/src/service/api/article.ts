import { basicRequest } from '../request';

/**
 * 获取文章列表
 */
export function fetchArticles() {
  return basicRequest.post<Blog.ArticlesInfo[]>('/getArticlesInfo', {});
}

/**
 * 获取文章编辑
 */
export function fetchArticleEditData(table_id: number) {
  return basicRequest.post<Blog.ArticlesEditInfo>('/getArticleEditData', { table_id });
}
