<template>
  <n-card :bordered="false" class="rounded-16px shadow-sm">
    <n-space vertical>
      <n-space class="p-l-30px">
        <n-gradient-text :size="24"> 文章管理 </n-gradient-text>
        <n-button type="primary">
          <icon-ic-round-plus class="mr-4px text-20px" />
          新增
        </n-button>
      </n-space>
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
          <n-grid class="p-t-5px p-b-5 p-l-30px p-r-30px" :x-gap="12" :y-gap="8" :cols="4">
            <n-gi v-for="n in 10" :key="n">
              <ImageOrTextCard
                :id="n"
                img-url="https://img0.baidu.com/it/u=4017997553,107592496&fm=253&fmt=auto&app=120&f=JPEG?w=1422&h=800"
              ></ImageOrTextCard>
            </n-gi>
          </n-grid>
        </n-layout>
        <!-- <n-layout-footer position="absolute" style="height: 64px; padding: 24px"> 城府路 </n-layout-footer> -->
      </n-layout>
    </n-space>
  </n-card>
</template>

<script setup lang="tsx">
import { ImageOrTextCard } from './components';
</script>
