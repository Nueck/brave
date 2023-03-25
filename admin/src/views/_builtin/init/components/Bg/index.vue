<template>
  <div class="absolute-lt z-1 wh-full overflow-hidden">
    <template v-if="bgImg">
      <n-image class="absolute-rt wh-full" src="/bg/bg.jpg" object-fit="fill" preview-disabled />
    </template>
  </div>
</template>

<script lang="ts" setup>
interface Props {
  bgImg: boolean;
}

defineProps<Props>();
</script>
~
<style scoped></style>
