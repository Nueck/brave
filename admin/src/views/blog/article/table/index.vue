<template>
  <n-card :bordered="false" title="文章管理" class="rounded-10px shadow-sm w-full h-full">
    <n-button type="primary" @click="handleAdd">
      <icon-ic-round-plus class="text-24px" />
      新增
    </n-button>
    <n-menu
      default-value="all"
      :value="articleStore.current_tag"
      mode="horizontal"
      :options="articles.menuOptions.value as MenuMixedOption[]"
      @update-value="switchTag"
    />

    <n-layout class="bg-transparent w-full h-600px">
      <n-layout embedded class="rounded-10px h-550px" has-sider>
        <n-layout-sider
          show-trigger
          collapse-mode="width"
          :collapsed="collapsed"
          :native-scrollbar="false"
          @collapse="collapsed = true"
          @expand="collapsed = false"
        >
          <n-menu
            default-value="all"
            :value="articleStore.current_tag"
            :options="articles.menuOptions.value as MenuMixedOption[]"
            @update-value="switchTag"
          />
        </n-layout-sider>
        <n-spin :show="articleStore.acticleLoading" class="h-full w-full">
          <template v-if="articles.articlesData.value.length === 0">
            <n-empty :description="description" class="absolute-center h-full">
              <template #extra>
                <n-button size="small" @click="handleAdd"> 点击新建 </n-button>
              </template>
            </n-empty>
          </template>
          <template v-else>
            <n-grid
              v-if="articles.articlesData.value"
              class="p-5"
              x-gap="8"
              y-gap="50"
              cols="1 200:1 400:2 600:3 800:4 1000:5 "
            >
              <n-gi v-for="(article, index) in articles.articlesData.value" :key="index">
                <template v-if="articleStore.acticleLoading">
                  <n-skeleton height="200px" :sharp="false" />
                </template>
                <template v-else>
                  <ImageOrTextCard
                    id="ImageOrTextCard"
                    :table-id="article.table_id"
                    :img-url="article.img_url"
                    :text="article.title"
                  />
                </template>
              </n-gi>
            </n-grid>
          </template>
        </n-spin>
      </n-layout>
      <n-space justify="end">
        <n-pagination
          class="h-20px p-t-5"
          :page="articles.current_page.value"
          :page-count="articles.page_total.value"
          @update-page="loadArticles"
        />
      </n-space>
    </n-layout>
  </n-card>
</template>

<script setup lang="tsx">
import { ref } from 'vue';
import { storeToRefs } from 'pinia';
import type { MenuOption } from 'naive-ui/es/menu/src/interface';
import { MenuMixedOption } from 'naive-ui/es/menu/src/interface';
import { useRouterPush } from '@/composables';
import { routeName } from '~/src/router';
import { useArticlesStore } from '~/src/store';
import { ImageOrTextCard } from './components';
const { routerPush } = useRouterPush();
const articleStore = useArticlesStore();
const articles = storeToRefs(articleStore);

const collapsed = ref(true);

const description = '你什么也找不到,试试新建一个文章';

function handleAdd() {
  routerPush({ name: routeName('blog_article_edit') });
}

// 加载资源
async function loadArticles(page: number) {
  await articleStore.getArticles(page);
}

async function switchTag(key: string, _item: MenuOption) {
  await articleStore.switchTags(key);
}

// 初始化数据
articleStore.getArticlesPageTotal();
loadArticles(1);
articleStore.getTags();
</script>
