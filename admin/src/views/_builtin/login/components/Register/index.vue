<template>
  <n-form ref="formRef" :model="model" :rules="rules" size="large" :show-label="false">
    <n-form-item path="username">
      <n-input v-model:value="model.username" placeholder="用户名" />
    </n-form-item>
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
    <n-form-item path="pwd">
      <n-input v-model:value="model.pwd" type="password" show-password-on="click" placeholder="密码" />
    </n-form-item>
    <n-form-item path="confirmPwd">
      <n-input v-model:value="model.confirmPwd" type="password" show-password-on="click" placeholder="确认密码" />
    </n-form-item>
    <n-space :vertical="true" :size="18">
      <login-agreement v-model:value="agreement" />
      <n-button type="primary" size="large" :block="true" :round="true" @click="handleSubmit">确定</n-button>
      <n-button size="large" :block="true" :round="true" @click="toLoginModule('pwd-login')">返回</n-button>
    </n-space>
  </n-form>
</template>

<script lang="ts" setup>
import { reactive, ref, toRefs } from 'vue';
import type { FormInst, FormRules } from 'naive-ui';
import { useRouterPush } from '@/composables';
import { useSmsCode } from '@/hooks';
import { formRules, getConfirmPwdRule } from '@/utils';
import { fetchRegister } from '~/src/service';
import { useAuthStore } from '~/src/store';

const { setTempInfoToLocal, removeTempInfoFormLocal } = useAuthStore();
const { toLoginModule } = useRouterPush();
const { label, isCounting, loading: smsLoading, getEmailCode, tokenCode } = useSmsCode();

const formRef = ref<HTMLElement & FormInst>();

const model = reactive({
  username: '',
  email: '',
  code: '',
  pwd: '',
  confirmPwd: ''
});

const rules: FormRules = {
  username: formRules.user,
  email: formRules.email,
  code: formRules.code,
  pwd: formRules.pwd,
  confirmPwd: getConfirmPwdRule(toRefs(model).pwd)
};

const agreement = ref(false);

async function handleSubmit() {
  await formRef.value?.validate();

  const info: ApiAuth.RegisterInfo = {
    username: model.username,
    email: model.email,
    password: model.pwd,
    verify_code: model.code,
    code: tokenCode.value
  };

  const { message } = await fetchRegister(info);
  /* 将数据获取 */
  if (message) {
    window.$message?.success('注册成功!');
    removeTempInfoFormLocal();
    const { username, pwd } = model;
    setTempInfoToLocal(username, pwd);

    setTimeout(() => {
      toLoginModule('pwd-login');
    }, 500);
  }
}
</script>

<style scoped></style>
