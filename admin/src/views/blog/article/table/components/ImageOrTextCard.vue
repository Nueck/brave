<template>
  <n-card class="max-w-68 max-h-50 rounded-10px border-1 shadow-sm hover:shadow-2xl" @click="clickCard">
    <template #cover>
      <n-image
        v-if="props.imgUrl !== ''"
        class="rounded-10px"
        preview-disabled
        object-fit="scale-down"
        :width="props.width"
        :height="props.height"
        :src="props.imgUrl"
        lazy
      />
    </template>

    <n-space class="m-t-3" align="center" justify="start">
      <n-ellipsis :line-clamp="2" class="max-h-auto">
        电灯熄灭 物换星移 泥牛入海黑暗好像 一颗巨石 按在胸口独脚大盗 百万富翁 摸爬滚打黑暗好像 一颗巨石 按在胸口
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
  width: number;
  height: number;
  text: string;
}

const props = withDefaults(defineProps<Props>(), {
  id: 12312,
  imgUrl: '',
  width: 400,
  height: 200,
  text: ''
});

function clickCard() {
  routerPush({ name: routeName('blog_article_edit'), query: { tableId: props.id } });
}

defineEmits(['clickCard']);
</script>
