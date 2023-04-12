<template>
  <n-card class="shadow-xl hover:shadow-2xl">
    <template #cover>
      <n-image class="w-auto h-35" :src="props.imgUrl" preview-disabled object-fit="fill" @click="clickCard"> </n-image>
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
  tableId: number;
  imgUrl: string;
  text: string;
}

const props = withDefaults(defineProps<Props>(), {
  tableId: 0,
  imgUrl: '',
  text: ''
});

async function deleteArticle() {
  await article.deleteData(props.tableId);
}

function clickCard() {
  routerPush({ name: routeName('blog_article_edit'), hash: `#${props.tableId}` });
}
</script>
