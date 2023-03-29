import { defineStore } from 'pinia';
import { fetchArticles } from '~/src/service/api/article';

interface BlogStore {
  articlesData: Blog.ArticlesInfo[];
  acticleLoading: boolean;
}

export const useBlogStore = defineStore('blog-store', {
  state: (): BlogStore => ({
    articlesData: [],
    acticleLoading: false
  }),
  actions: {
    async getArticles() {
      this.acticleLoading = true;
      const { data } = await fetchArticles();
      if (data) {
        (this.articlesData as Blog.ArticlesInfo[]) = data;
      } else {
        this.articlesData = <Blog.ArticlesInfo[]>[];
      }
      this.acticleLoading = false;
    }
  }
});
