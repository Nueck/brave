import { basicRequest } from '../request';

/**
 * 获取文章列表
 */
export function fetchArticles() {
  return basicRequest.post<Blog.ArticlesInfo[]>('/getArticlesInfo', {});
}
