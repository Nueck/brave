<template>
  <n-card :bordered="false" title="相册管理" class="rounded-10px shadow-sm w-full h-full">
    <n-button type="primary">
      <icon-ic-round-plus class="text-24px" />
      新增
    </n-button>
    <n-spin :show="false">
      <n-layout class="bg-transparent w-full h-600px">
        <template v-if="true">
          <n-empty :description="description" class="absolute-center h-full">
            <template #extra>
              <n-button size="small"> 点击新建 </n-button>
            </template>
          </n-empty>
        </template>
        <template v-else>
          <!-- <n-layout embedded class="rounded-10px h-550px">
            <n-grid
              v-if="articles.articlesData.value"
              class="p-5"
              x-gap="8"
              y-gap="50"
              cols="1 200:1 400:2 600:3 800:4 1000:5 "
            >
              <n-gi v-for="(article, index) in articles.articlesData.value" :key="index">
                <ImageCard
                  id="ImageOrTextCard"
                  :table-id="article.table_id"
                  :img-url="article.img_url"
                  :text="article.title"
                ></ImageCard>
              </n-gi>
            </n-grid>
          </n-layout> -->
          <!-- <n-space justify="end">
            <n-pagination
              class="h-20px p-t-5"
              :page="articles.current_page.value"
              :page-count="articles.page_total.value"
              @update-page="loadArticles"
            />
          </n-space> -->
        </template>
      </n-layout>
    </n-spin>
  </n-card>
</template>

<script setup lang="tsx">
// import { ImageCard } from './components';

const description = '你什么也找不到,试试新建一个图片';
</script>
