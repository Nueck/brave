<template>
  <n-card class="border-1 shadow-sm hover:shadow-2xl">
    <template #cover>
      <n-image class="w-auto h-24" :src="props.imgUrl" object-fit="fill" @click="clickCard"> </n-image>
    </template>
    <n-ellipsis :line-clamp="1" class="h-auto">
      {{ text }}
    </n-ellipsis>
    <n-space justify="space-between">
      <n-button strong secondary round size="small" @click="clickCard">编辑</n-button>
      <n-popconfirm @positive-click="deleteArticle()">
        <template #trigger>
          <n-button strong secondary round size="small">删除</n-button>
        </template>
        是否删除
      </n-popconfirm>
    </n-space>
  </n-card>
</template>

<script setup lang="tsx">
import { routeName } from '@/router';
import { useRouterPush } from '@/composables';
import { useArticlesStore } from '~/src/store';
const { routerPush } = useRouterPush();
const article = useArticlesStore();

interface Props {
  id: number;
  imgUrl: string;
  text: string;
}

const props = withDefaults(defineProps<Props>(), {
  id: 0,
  imgUrl: '',
  text: ''
});

async function deleteArticle() {
  await article.deleteData(props.id);
  await article.getArticles();
}

function clickCard() {
  routerPush({ name: routeName('blog_article_edit'), query: { tableId: props.id } });
}
</script>
