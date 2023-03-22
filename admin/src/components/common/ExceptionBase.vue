<template>
  <div class="flex-col-center wh-full">
    <div class="text-400px text-primary">
      <icon-local-no-permission v-if="type === '403'" />
      <icon-local-not-found v-if="type === '404'" />
      <icon-local-service-error v-if="type === '500'" />
    </div>
    <n-button type="primary" @click="toHome()">回到首页</n-button>
  </div>
</template>

<script lang="ts" setup>
import { useRouterPush } from '@/composables';
const { toHome } = useRouterPush();

defineOptions({ name: 'ExceptionBase' });

type ExceptionType = '403' | '404' | '500';

interface Props {
  /** 异常类型 403 404 500 */
  type: ExceptionType;
}

defineProps<Props>();
</script>

<style scoped></style>
