<template>
  <n-card :bordered="false" class="rounded-16px shadow-sm">
    <n-space vertical>
      <n-space class="w-full">
        <n-space class="p-l-30px">
          <n-gradient-text :size="24"> 文章管理 </n-gradient-text>
          <n-button type="primary" @click="handleAdd">
            <icon-ic-round-plus class="mr-4px text-20px" />
            新增
          </n-button>
          <n-button type="primary" class="m-r-30 absolute-rt m-t-5" @click="blog.getArticles">
            <icon-mdi-refresh :class="{ 'animate-spin': blog.acticleLoading }" />
          </n-button>
        </n-space>
      </n-space>
      <n-space vertical>
        <n-layout class="rounded-16px bg-transparent" has-sider>
          <!-- <n-layout-sider
              content-style="padding: 24px;"
              collapse-mode="width"
              :collapsed-width="120"
              :width="240"
              show-trigger="arrow-circle"
              :native-scrollbar="false"
              bordered
            >
            </n-layout-sider> -->
          <n-layout :native-scrollbar="false">
            <n-grid
              v-if="articles.articlesData.value"
              class="p-t-5px p-b-5 p-l-30px p-r-10px"
              :x-gap="12"
              :y-gap="8"
              :cols="4"
            >
              <n-gi v-for="(article, index) in articles.articlesData.value" :key="index">
                <ImageOrTextCard
                  :id="article.table_id"
                  :img-url="article.img_url"
                  :text="article.title"
                ></ImageOrTextCard>
              </n-gi>
            </n-grid>
          </n-layout>
          <!-- <n-layout-footer position="absolute" style="height: 64px; padding: 24px"> 城府路 </n-layout-footer> -->
        </n-layout>
      </n-space>
    </n-space>
  </n-card>
</template>

<script setup lang="tsx">
import { storeToRefs } from 'pinia';
import { useRouterPush } from '@/composables';
import { useBlogStore } from '~/src/store';
import { routeName } from '~/src/router';
import { ImageOrTextCard } from './components';
const { routerPush } = useRouterPush();
const blog = useBlogStore();
const articles = storeToRefs(blog);
blog.getArticles();

function handleAdd() {
  routerPush({ name: routeName('blog_article_edit') });
}
</script>
