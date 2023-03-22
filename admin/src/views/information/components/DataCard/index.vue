<template>
  <n-grid cols="s:1 m:2 l:4" responsive="screen" :x-gap="16" :y-gap="16">
    <n-grid-item v-for="item in cardData" :key="item.id">
      <gradient-bg class="h-100px" :start-color="item.colors[0]" :end-color="item.colors[1]">
        <h3 class="text-16px">{{ item.title }}</h3>
        <div class="flex justify-between pt-12px">
          <svg-icon :icon="item.icon" class="text-32px" />
          <count-to
            :prefix="item.unit"
            :start-value="1"
            :end-value="item.value"
            class="text-30px text-white dark:text-dark"
          />
        </div>
      </gradient-bg>
    </n-grid-item>
  </n-grid>
</template>

<script setup lang="ts">
import { GradientBg } from './components';

defineOptions({ name: 'DashboardAnalysisDataCard' });

interface CardData {
  id: string;
  title: string;
  value: number;
  unit: string;
  colors: [string, string];
  icon: string;
}

const cardData: CardData[] = [
  {
    id: 'visit',
    title: '访问量',
    value: 1000,
    unit: '',
    colors: ['#ec4786', '#b955a4'],
    icon: 'ant-design:bar-chart-outlined'
  },
  {
    id: 'read',
    title: '阅读量',
    value: 23,
    unit: '',
    colors: ['#865ec0', '#5144b4'],
    icon: 'ri:book-read-line'
  },
  {
    id: 'message',
    title: '留言量',
    value: 50,
    unit: '',
    colors: ['#56cdf3', '#719de3'],
    icon: 'mdi:message-badge-outline'
  },
  {
    id: 'books',
    title: '文章数',
    value: 99,
    unit: '',
    colors: ['#fcbc25', '#f68057'],
    icon: 'ph:books-fill'
  }
];
</script>

<style scoped></style>
