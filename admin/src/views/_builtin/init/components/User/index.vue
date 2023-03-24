<template>
  <n-form ref="formRef" :model="model" :rules="rules" size="large" :show-label="false">
    <n-form-item path="username">
      <n-input v-model:value="model.username" placeholder="用户名" />
    </n-form-item>
    <n-form-item path="email">
      <n-input v-model:value="model.email" placeholder="邮箱地址" />
    </n-form-item>
    <n-form-item path="pwd">
      <n-input v-model:value="model.pwd" type="password" show-password-on="click" placeholder="密码" />
    </n-form-item>
    <n-form-item path="confirmPwd">
      <n-input v-model:value="model.confirmPwd" type="password" show-password-on="click" placeholder="确认密码" />
    </n-form-item>
    <n-form-item path="address">
      <n-input v-model:value="model.address" placeholder="网站信息(格式XXX.com)" />
    </n-form-item>
    <n-space :vertical="true" :size="18">
      <login-agreement v-model:value="agreement" />
      <n-button type="primary" size="large" :block="true" :round="true" @click="handleSubmit">确定</n-button>
    </n-space>
  </n-form>
</template>

<script lang="ts" setup>
import { reactive, ref, toRefs } from 'vue';
import type { FormInst, FormRules } from 'naive-ui';
import { formRules, getConfirmPwdRule } from '@/utils';
import { fetchInit } from '~/src/service';
import { useAuthStore, useInitStore } from '~/src/store';
const auth = useAuthStore();
const init = useInitStore();

const formRef = ref<HTMLElement & FormInst>();

const model = reactive({
  username: '',
  email: '',
  pwd: '',
  confirmPwd: '',
  address: ''
});

const rules: FormRules = {
  username: formRules.user,
  email: formRules.email,
  pwd: formRules.pwd,
  confirmPwd: getConfirmPwdRule(toRefs(model).pwd)
};

const agreement = ref(false);

async function handleSubmit() {
  await formRef.value?.validate();

  const info: Init.InitInfo = {
    username: model.username,
    email: model.email,
    password: model.pwd,
    address: model.address
  };

  const { data } = await fetchInit(info);
  /* 将数据获取 */
  if (data) {
    await auth.login(info.username, info.password);
    await init.initStatusStore();

    setTimeout(() => {
      window.$message?.success('初始化成功!');
    }, 500);
  }
}
</script>

<style scoped></style>
