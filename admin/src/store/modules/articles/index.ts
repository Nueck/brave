import type { Component } from 'vue';
import { defineStore } from 'pinia';
import { useIconRender, useRouterPush } from '~/src/composables';
import { routeName } from '~/src/router';
import {
  fetchArticles,
  fetchArticlesPageTotal,
  fetchDeleteArticleData,
  fetchSaveArticleEditData,
  fetchUpdateArticleEditData
} from '~/src/service/api/article';
import { fetchTags } from '~/src/service/api/tags';
import { useTabStore } from '../tab';
const { iconRender } = useIconRender();

interface MenuOption {
  label: string;
  key: string;
  icon?: Component;
}

interface ArticlesStore {
  menuOptions: MenuOption[];
  articlesData: Blog.ArticlesInfo[];
  acticleLoading: boolean;
  loading: boolean;
  page_total: number;
  current_page: number;
  current_tag: string;
}

export const useArticlesStore = defineStore('articles-store', {
  state: (): ArticlesStore => ({
    menuOptions: [],
    articlesData: [],
    acticleLoading: false,
    loading: false,
    page_total: 1,
    current_page: 1,
    current_tag: 'all'
  }),
  actions: {
    async getArticles(page: number) {
      this.acticleLoading = true;
      if (page > 0) {
        const { data } = await fetchArticles(page - 1, this.current_tag);
        if (data) {
          (this.articlesData as Blog.ArticlesInfo[]) = data;
          this.current_page = page;
        } else {
          this.articlesData = <Blog.ArticlesInfo[]>[];
        }
      }
      setTimeout(() => {
        this.acticleLoading = false;
      }, 300);
    },

    async getArticlesPageTotal() {
      const { data } = await fetchArticlesPageTotal(this.current_tag);
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
        }, 100);
      }
    },
    // 保存数据
    async saveData(data: Blog.ArticleEditData, fullPath: string) {
      if (!this.loading) {
        this.loading = true;

        const { error } = await fetchSaveArticleEditData(data);
        if (!error) {
          // 重新加载页数
          await this.getArticles(this.current_page);
          const tab = useTabStore();
          await tab.removeTab(fullPath);

          window.$message?.success('保存数据成功');
        }
        setTimeout(() => {
          this.loading = false;
        }, 100);
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
    },
    async getTags() {
      const { data } = await fetchTags();
      this.menuOptions = [];
      this.menuOptions.push({
        label: '全部',
        key: 'all',
        icon: iconRender({ icon: 'material-symbols:border-all-rounded' })
      });
      if (data) {
        data.forEach(value => {
          const menu: MenuOption = {
            label: value,
            key: value,
            icon: iconRender({ icon: 'material-symbols:border-all-rounded' })
          };
          this.menuOptions.push(menu);
        });
      }
    },
    async switchTags(tag: string) {
      this.current_tag = tag;
      await this.getArticlesPageTotal();
      await this.getArticles(this.current_page);
    }
  }
});
