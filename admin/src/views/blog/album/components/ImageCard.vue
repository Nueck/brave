<template>
  <n-card class="shadow-sm hover:shadow-2xl">
    <template #cover>
      <n-image class="w-auto h-35" :src="props.imgUrl" preview-disabled object-fit="fill"> </n-image>
    </template>
    <n-space justify="space-between">
      <n-button strong secondary round size="small">编辑</n-button>
      <n-popconfirm>
        <template #trigger>
          <n-button strong secondary round size="small">删除</n-button>
        </template>
        是否删除
      </n-popconfirm>
    </n-space>
  </n-card>
</template>

<script setup lang="tsx">
interface Props {
  imgUrl: string;
  text: string;
}

const props = withDefaults(defineProps<Props>(), {
  imgUrl: '',
  text: ''
});
</script>
