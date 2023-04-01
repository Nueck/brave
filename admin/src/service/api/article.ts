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

/**
 * 更新文章数据
 */
export function fetchUpdateArticleEditData(data: Blog.ArticleEditData) {
  return basicRequest.post('/updateArticleEditData', data);
}

/**
 * 保存文章数据
 */
export function fetchSaveArticleEditData(data: Blog.ArticleEditData) {
  return basicRequest.post('/saveArticleEditData', data);
}

/**
 * 删除文章文章数据
 */
export function fetchDeleteArticleData(table_id: number) {
  return basicRequest.post('/deleteArticleData', { table_id });
}
