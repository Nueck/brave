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
    <n-form-item path="pwd">
      <n-input v-model:value="model.pwd" type="password" show-password-on="click" placeholder="新密码" />
    </n-form-item>
    <n-form-item path="confirmPwd">
      <n-input v-model:value="model.confirmPwd" type="password" show-password-on="click" placeholder="确认密码" />
    </n-form-item>
    <n-space :vertical="true" size="large">
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
import { fetchForget } from '~/src/service';
import { useAuthStore } from '~/src/store';

const { toLoginModule } = useRouterPush();
const { label, isCounting, loading: smsLoading, getEmailCode, tokenCode } = useSmsCode();
const { resetAuthStore, removeTempInfoFormLocal } = useAuthStore();

const formRef = ref<HTMLElement & FormInst>();

const model = reactive({
  email: '',
  code: '',
  pwd: '',
  confirmPwd: ''
});

const rules: FormRules = {
  email: formRules.email,
  code: formRules.code,
  pwd: formRules.pwd,
  confirmPwd: getConfirmPwdRule(toRefs(model).pwd)
};

async function handleSubmit() {
  await formRef.value?.validate();

  await formRef.value?.validate();

  const info: ApiAuth.ForgetInfo = {
    email: model.email,
    new_pwd: model.pwd,
    verify_code: model.code,
    code: tokenCode.value
  };

  const { message } = await fetchForget(info);
  /* 将数据获取 */
  if (message) {
    removeTempInfoFormLocal();
    resetAuthStore();
    window.$message?.success('修改密码成功!');
    window.$message?.success('请使用新密码登陆!');

    setTimeout(() => {
      toLoginModule('pwd-login');
    }, 500);
  } else {
    window.$message?.success('修改密码失败!');
  }
}
</script>

<style scoped></style>
