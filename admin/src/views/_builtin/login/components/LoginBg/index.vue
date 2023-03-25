<template>
  <div class="absolute-lt z-1 wh-full overflow-hidden" style="user-select: none">
    <template v-if="bgImg">
      <n-image class="absolute-lt wh-full" :src="imgUrl" object-fit="fill" preview-disabled />
    </template>
    <template v-else>
      <div class="absolute -right-300px -top-900px <sm:(-right-100px -top-1170px)">
        <corner-top :start-color="lightColor" :end-color="darkColor" />
      </div>
      <div class="absolute -left-200px -bottom-400px <sm:(-left-100px -bottom-760px)">
        <corner-bottom :start-color="darkColor" :end-color="lightColor" />
      </div>
    </template>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { getColorPalette } from '@/utils';
import imgUrl from '@/assets/bg/bg.jpg';
import { CornerBottom, CornerTop } from './components';

interface Props {
  /** 主题颜色 */
  themeColor: string;
  bgImg: boolean;
}

const props = defineProps<Props>();

const lightColor = computed(() => getColorPalette(props.themeColor, 3));
const darkColor = computed(() => getColorPalette(props.themeColor, 6));
</script>

<style scoped></style>
