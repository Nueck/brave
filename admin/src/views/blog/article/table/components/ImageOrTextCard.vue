<template>
  <n-card class="max-w-68 max-h-50 rounded-10px border-1 shadow-sm hover:shadow-2xl" @click="clickCard">
    <template #cover>
      <n-image
        class="rounded-10px"
        preview-disabled
        object-fit="scale-down"
        :width="300"
        :height="200"
        :src="props.imgUrl"
        lazy
      />
    </template>

    <n-space class="m-t-3" align="center" justify="start">
      <n-ellipsis :line-clamp="2" class="max-h-auto">
        {{ text }}
      </n-ellipsis>
    </n-space>
  </n-card>
</template>

<script setup lang="tsx">
import { routeName } from '@/router';
import { useRouterPush } from '@/composables';
const { routerPush } = useRouterPush();

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

function clickCard() {
  routerPush({ name: routeName('blog_article_edit'), query: { tableId: props.id } });
}
</script>
