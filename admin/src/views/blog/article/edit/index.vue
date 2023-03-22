<template>
  <n-space :vertical="true" :size="16">
    <n-button @click="handleToTab">返回</n-button>
    <md-editor v-model="text" />
  </n-space>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import MdEditor from 'md-editor-v3';
import { routeName } from '@/router';
import 'md-editor-v3/lib/style.css';
import { useRouterPush } from '@/composables';

const { routerPush } = useRouterPush();

const text = ref('# Hello Editor');

function handleToTab() {
  routerPush({ name: routeName('blog_article_table') });
}
</script>

<style scoped></style>
