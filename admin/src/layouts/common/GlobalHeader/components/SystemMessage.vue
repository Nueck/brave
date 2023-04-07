<template>
  <n-popover class="!p-0" trigger="click" placement="bottom">
    <template #trigger>
      <hover-container tooltip-content="消息通知" :inverted="theme.header.inverted" class="relative w-40px h-full">
        <icon-clarity:notification-line class="text-18px" />
        <n-badge
          :value="count"
          :max="99"
          :class="[count < 10 ? '-right-2px' : '-right-10px']"
          class="absolute top-10px"
        />
      </hover-container>
    </template>
    <n-tabs
      v-model:value="currentTab"
      :class="[isMobile ? 'w-276px' : 'w-360px']"
      type="line"
      justify-content="space-evenly"
    >
      <n-tab-pane v-for="(item, index) in tabData" :key="item.key" :name="index">
        <template #tab>
          <div class="flex-x-center items-center" :class="[isMobile ? 'w-92px' : 'w-120px']">
            <span class="mr-5px">{{ item.name }}</span>
            <n-badge
              v-bind="item.badgeProps"
              :value="item.list.filter(message => !message.isRead).length"
              :max="99"
              show-zero
            />
          </div>
        </template>
        <loading-empty-wrapper
          class="h-360px"
          :loading="loading"
          :empty="item.list.length === 0"
          placeholder-class="bg-$n-color transition-background-color duration-300 ease-in-out"
        >
          <message-list :list="item.list" @read="handleRead" />
        </loading-empty-wrapper>
      </n-tab-pane>
    </n-tabs>
    <div v-if="showAction" class="flex border-t border-$n-divider-color cursor-pointer">
      <div class="flex-1 text-center py-10px" @click="handleClear">清空</div>
      <div class="flex-1 text-center py-10px border-l border-$n-divider-color" @click="handleAllRead">全部已读</div>
      <div class="flex-1 text-center py-10px border-l border-$n-divider-color" @click="handleLoadMore">查看更多</div>
    </div>
  </n-popover>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { useThemeStore } from '@/store';
import { useBasicLayout } from '@/composables';
import { useBoolean } from '@/hooks';
import MessageList from './MessageList.vue';

defineOptions({ name: 'SystemMessage' });

const theme = useThemeStore();
const { isMobile } = useBasicLayout();
const { bool: loading, setBool: setLoading } = useBoolean();

const currentTab = ref(0);

const tabData = ref<App.MessageTab[]>([
  {
    key: 1,
    name: '通知',
    badgeProps: { type: 'warning' },
    list: [{ id: 1, icon: 'ri:message-3-line', title: '你收到了5条新消息', date: '2022-06-17' }]
  },
  {
    key: 2,
    name: '消息',
    badgeProps: { type: 'error' },
    list: [
      {
        id: 1,
        title: '项目动态',
        svgIcon: 'avatar',
        description: 'Soybean 刚才把工作台页面随便写了一些，凑合能看了！',
        date: '2021-11-07 22:45:32'
      }
    ]
  },
  {
    key: 3,
    name: '待办',
    badgeProps: { type: 'info' },
    list: [
      {
        id: 1,
        icon: 'ri:calendar-todo-line',
        title: '缓存主题配置',
        description: '任务正在计划中',
        date: '2022-06-17',
        tagTitle: '未开始',
        tagProps: { type: 'default' }
      }
    ]
  }
]);

const count = computed(() => {
  return tabData.value.reduce((acc, cur) => {
    return acc + cur.list.filter(item => !item.isRead).length;
  }, 0);
});

const showAction = computed(() => tabData.value[currentTab.value].list.length > 0);

function handleRead(index: number) {
  tabData.value[currentTab.value].list[index].isRead = true;
}

function handleAllRead() {
  tabData.value[currentTab.value].list.forEach(item => Object.assign(item, { isRead: true }));
}

function handleClear() {
  tabData.value[currentTab.value].list = [];
}

function handleLoadMore() {
  const { list } = tabData.value[currentTab.value];
  setLoading(true);
  setTimeout(() => {
    list.push(...tabData.value[currentTab.value].list);
    setLoading(false);
  }, 1000);
}
</script>
<style scoped></style>
