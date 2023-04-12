<template>
  <n-card :bordered="false" title="文章管理" class="rounded-10px shadow-sm w-full h-full">
    <n-button type="primary" @click="handleAdd">
      <icon-ic-round-plus class="text-24px" />
      新增
    </n-button>
    <n-spin :show="articleStore.acticleLoading">
      <n-layout class="bg-transparent w-full h-600px">
        <template v-if="articles.articlesData.value.length === 0">
          <n-empty :description="description" class="absolute-center h-full">
            <template #extra>
              <n-button size="small" @click="handleAdd"> 点击新建 </n-button>
            </template>
          </n-empty>
        </template>

        <template v-else>
          <n-layout content-style="w-full ">
            <n-grid
              v-if="articles.articlesData.value"
              class="p-b-5 p-t-5 h-570px"
              x-gap="8"
              y-gap="8"
              cols="1 150:1 300:2 450:3 600:4 750:5 900:6"
            >
              <n-gi v-for="(article, index) in articles.articlesData.value" :key="index">
                <ImageOrTextCard :id="article.table_id" :img-url="article.img_url" :text="article.title" />
              </n-gi>
            </n-grid>
          </n-layout>
          <n-space justify="end">
            <n-pagination v-model:page="page" class="h-30px" :page-count="1" />
          </n-space>
        </template>
      </n-layout>
    </n-spin>
  </n-card>
</template>

<script setup lang="tsx">
import { ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useRouterPush } from '@/composables';
import { routeName } from '~/src/router';
import { useArticlesStore } from '~/src/store';
import { ImageOrTextCard } from './components';
const { routerPush } = useRouterPush();
const articleStore = useArticlesStore();

const articles = storeToRefs(articleStore);
const page = ref(1);
const description = '你什么也找不到,试试新建一个文章';

function handleAdd() {
  routerPush({ name: routeName('blog_article_edit') });
}

// 加载资源
articleStore.getArticles();
</script>
