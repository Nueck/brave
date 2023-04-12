import { defineStore } from 'pinia';
import { useRouterPush } from '~/src/composables';
import { routeName } from '~/src/router';
import {
  fetchArticles,
  fetchArticlesPageTotal,
  fetchDeleteArticleData,
  fetchSaveArticleEditData,
  fetchUpdateArticleEditData
} from '~/src/service/api/article';

interface ArticlesStore {
  articlesData: Blog.ArticlesInfo[];
  acticleLoading: boolean;
  loading: boolean;
  page_total: number;
  current_page: number;
}

export const useArticlesStore = defineStore('articles-store', {
  state: (): ArticlesStore => ({
    articlesData: [],
    acticleLoading: false,
    loading: false,
    page_total: 1,
    current_page: 1
  }),
  actions: {
    async getArticles(page: number) {
      this.acticleLoading = true;
      if (page > 0) {
        const { data } = await fetchArticles(page - 1);
        if (data) {
          (this.articlesData as Blog.ArticlesInfo[]) = data;
        } else {
          this.articlesData = <Blog.ArticlesInfo[]>[];
        }
      }
      setTimeout(() => {
        this.acticleLoading = false;
      }, 300);
    },

    async getArticlesPageTotal() {
      const { data } = await fetchArticlesPageTotal();
      if (data) {
        this.page_total = data.page_total;
      } else {
        this.page_total = 1;
      }
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
          // 重新加载页数
          await this.getArticlesPageTotal();
          await this.getArticles(this.current_page);
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
          await this.getArticlesPageTotal();
          await this.getArticles(this.current_page);
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
