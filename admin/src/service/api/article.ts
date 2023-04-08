import { basicRequest } from '../request';

/**
 * 获取文章列表
 */
export function fetchArticles() {
  return basicRequest.get<Blog.ArticlesInfo[]>('/articles');
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
