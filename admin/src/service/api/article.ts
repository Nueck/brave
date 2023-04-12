import { basicRequest } from '../request';

/**
 * 获取文章列表
 */
export function fetchArticles(page: number) {
  return basicRequest.get<Blog.ArticlesInfo[]>(`/articles/${page}`);
}

/**
 * 获取文章列表总页数
 */
export function fetchArticlesPageTotal() {
  return basicRequest.get<Blog.ArticlesPage>('/articles/page');
}

/**
 * 获取文章编辑
 */
export function fetchArticleEditData(table_id: number) {
  return basicRequest.get<Blog.ArticlesEditInfo>(`/article/${table_id}`);
}

/**
 * 更新文章数据
 */
export function fetchUpdateArticleEditData(data: Blog.ArticleEditData) {
  return basicRequest.put('/article', data);
}

/**
 * 保存文章数据
 */
export function fetchSaveArticleEditData(data: Blog.ArticleEditData) {
  return basicRequest.post('/article', data);
}

/**
 * 删除文章文章数据
 */
export function fetchDeleteArticleData(table_id: number) {
  return basicRequest.delete(`/article/${table_id}`, {});
}
