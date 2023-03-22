<template>
  <n-form ref="formRef" :model="model" :rules="rules" size="large" :show-label="false">
    <n-form-item path="user">
      <n-input v-model:value="model.userName" placeholder="请输入用户名或邮箱" />
    </n-form-item>
    <n-form-item path="password">
      <n-input v-model:value="model.password" type="password" show-password-on="click" placeholder="请输入密码" />
    </n-form-item>
    <n-space :vertical="true" :size="24">
      <div class="flex-y-center justify-between">
        <n-checkbox v-model:checked="rememberMe" @update:checked="handleUpdateValue">记住我</n-checkbox>
        <n-button :text="true" @click="toLoginModule('reset-pwd')">忘记密码？</n-button>
      </div>
      <n-button type="primary" size="large" :block="true" :round="true" :loading="loginLoading" @click="handleSubmit">
        登陆
      </n-button>
      <div class="flex-y-center justify-between">
        <n-button class="flex-1" :block="true" @click="toLoginModule('code-login')">
          {{ EnumLoginModule['code-login'] }}
        </n-button>
        <div class="w-12px"></div>
        <n-button class="flex-1" :block="true" @click="toLoginModule('register')">
          {{ EnumLoginModule.register }}
        </n-button>
      </div>
    </n-space>
  </n-form>
</template>

<script setup lang="ts">
import { reactive, ref, watch } from 'vue';
import type { FormInst, FormRules } from 'naive-ui';
import { EnumLoginModule } from '@/enum';
import { useAuthStore } from '@/store';
import { useRouterPush } from '@/composables';
import { formRules } from '@/utils';

const { login, tempInfo, setTempInfoToLocal, removeTempInfoFormLocal, loginLoading } = useAuthStore();
const { toLoginModule } = useRouterPush();

const formRef = ref<HTMLElement & FormInst>();

// 判断临时信息的
const model = reactive({
  userName: tempInfo ? tempInfo.userName : '',
  password: tempInfo ? tempInfo.userPwd : ''
});

const rememberMe = ref(Boolean(tempInfo));

const rules: FormRules = {
  password: formRules.pwd
};

async function handleSubmit() {
  await formRef.value?.validate();

  const { userName, password } = model;

  login(userName, password);
}

// 处理复选框的事件
function handleUpdateValue(value: boolean) {
  rememberMe.value = value;
  if (value) {
    const { userName, password } = model;
    setTempInfoToLocal(userName, password);
  } else {
    removeTempInfoFormLocal();
  }
}
// 监听数据变化，如果有输入将记住我关闭
watch(model, () => {
  rememberMe.value = false;
});
</script>

<style scoped></style>
