<template>
  <div class="relative flex-center wh-full" :style="{ backgroundColor: bgColor }">
    <dark-mode-switch
      :dark="theme.darkMode"
      class="absolute left-48px top-24px z-3 text-20px"
      @update:dark="theme.setDarkMode"
    />
    <n-card :bordered="false" size="large" class="z-4 !w-auto rounded-5px op-95px shadow-sm">
      <div class="w-300px sm:w-360px">
        <header class="flex-y-center justify-between">
          <n-gradient-text style="user-select: none" type="primary" :size="28">系统初始化</n-gradient-text>
        </header>
        <main class="pt-24px">
          <h3 style="user-select: none" class="text-18px text-primary font-medium">{{ activeModule.label }}</h3>
          <div class="pt-24px">
            <transition name="fade-slide" mode="out-in" appear>
              <component :is="activeModule.component" style="user-select: none" />
            </transition>
          </div>
        </main>
      </div>
    </n-card>
    <bg style="user-select: none" :bg-img="true" :theme-color="bgThemeColor" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useThemeStore } from '@/store';
// import { useAppInfo } from '@/composables';
import { getColorPalette, mixColor } from '@/utils';
import { Bg, activeModule } from './components';

const theme = useThemeStore();
// const { title } = useAppInfo();

const bgThemeColor = computed(() => (theme.darkMode ? getColorPalette(theme.themeColor, 7) : theme.themeColor));

const bgColor = computed(() => {
  const COLOR_WHITE = '#ffffff';
  const ratio = theme.darkMode ? 0.5 : 0.2;
  return mixColor(COLOR_WHITE, theme.themeColor, ratio);
});
</script>

<style scoped></style>
