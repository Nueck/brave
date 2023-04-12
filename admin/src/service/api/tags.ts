import { basicRequest } from '../request';

/**
 * 获取标签列表
 */
export function fetchTags() {
  return basicRequest.get<string[]>('/tags');
}

/**
 * 保存新标签数据
 */
export function fetchSaveArticleEditData(tag: string) {
  return basicRequest.post('/tags', { tag });
}

/**
 * 删除指定标签数据
 */
export function fetchDeleteTag(tag: string) {
  return basicRequest.delete(`/tags/${tag}`, {});
}
