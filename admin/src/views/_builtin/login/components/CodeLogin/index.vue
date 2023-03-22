<template>
  <n-form ref="formRef" :model="model" :rules="rules" size="large" :show-label="false">
    <n-form-item path="email">
      <n-input v-model:value="model.email" placeholder="邮箱地址" />
    </n-form-item>
    <n-form-item path="code">
      <div class="flex-y-center w-full">
        <n-input v-model:value="model.code" placeholder="验证码" />
        <div class="w-18px"></div>
        <n-button size="large" :disabled="isCounting" :loading="smsLoading" @click="getEmailCode(model.email)">
          {{ label }}
        </n-button>
      </div>
    </n-form-item>
    <n-form-item path="imgCode">
      <n-input v-model:value="model.imgCode" placeholder="验证码,点击图片刷新" />
      <div class="pl-8px">
        <image-verify v-model:code="imgCode" />
      </div>
    </n-form-item>
    <n-space :vertical="true" :size="18">
      <n-button
        type="primary"
        size="large"
        :block="true"
        :round="true"
        :loading="auth.loginLoading"
        @click="handleSubmit"
      >
        登陆
      </n-button>
      <n-button size="large" :block="true" :round="true" @click="toLoginModule('pwd-login')">返回</n-button>
    </n-space>
  </n-form>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import type { FormInst } from 'naive-ui';
import { useAuthStore } from '@/store';
import { useSmsCode } from '@/hooks';
import { formRules, getImgCodeRule } from '@/utils';
import { toLoginModule } from '../index';

const auth = useAuthStore();

const { label, isCounting, loading: smsLoading, tokenCode, getEmailCode } = useSmsCode();

const formRef = ref<HTMLElement & FormInst>();

const model = reactive({
  email: '',
  code: '',
  imgCode: ''
});

const imgCode = ref('');

const rules = {
  email: formRules.email,
  code: formRules.code,
  imgCode: getImgCodeRule(imgCode)
};

async function handleSubmit() {
  await formRef.value?.validate();

  const { email, code } = model;

  auth.email_login(email, code, tokenCode.value);
}
</script>

<style scoped></style>
