import { defineStore } from 'pinia';
import { useRouterPush } from '~/src/composables';
import { routeName } from '~/src/router';
import {
  fetchArticles,
  fetchDeleteArticleData,
  fetchSaveArticleEditData,
  fetchUpdateArticleEditData
} from '~/src/service/api/article';

interface ArticlesStore {
  articlesData: Blog.ArticlesInfo[];
  acticleLoading: boolean;
  loading: boolean;
}

export const useArticlesStore = defineStore('articles-store', {
  state: (): ArticlesStore => ({
    articlesData: [],
    acticleLoading: false,
    loading: false
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
    },
    async updateData(data: Blog.ArticleEditData) {
      if (!this.loading) {
        this.loading = true;
        const { error } = await fetchUpdateArticleEditData(data);
        if (!error) {
          window.$message?.success('更新数据成功');
        }
        setTimeout(() => {
          this.loading = false;
        }, 500);
      }
    },
    // 保存数据
    async saveData(data: Blog.ArticleEditData) {
      if (!this.loading) {
        this.loading = true;

        const { error } = await fetchSaveArticleEditData(data);
        if (!error) {
          window.$message?.success('保存数据成功');
        }
        setTimeout(() => {
          this.loading = false;
        }, 500);
      }
    },
    async deleteData(data: number) {
      const { routerPush } = useRouterPush();
      if (!this.loading) {
        this.loading = true;
        const { error } = await fetchDeleteArticleData(data);
        if (!error) {
          window.$message?.success('删除数据成功');
        }

        setTimeout(() => {
          this.loading = false;
          routerPush({ name: routeName('blog_article_table') });
        }, 200);
      }
    }
  }
});
