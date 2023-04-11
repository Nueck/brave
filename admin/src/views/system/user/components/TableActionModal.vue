<template>
  <n-modal v-model:show="modalVisible" preset="card" :title="title" class="w-700px">
    <n-form ref="formRef" label-placement="left" :label-width="80" :model="formModel" :rules="rules">
      <n-grid :cols="24" :x-gap="18">
        <n-form-item-grid-item :span="12" label="用户名" path="userName">
          <n-input v-model:value="formModel.user_name" />
        </n-form-item-grid-item>
        <n-form-item-grid-item :span="12" label="权限" path="age">
          <n-select v-model:value="formModel.authority" :options="authorityOptions(formModel.authority)" />
        </n-form-item-grid-item>
        <n-form-item-grid-item :span="12" label="邮箱" path="email">
          <n-input v-model:value="formModel.email" />
        </n-form-item-grid-item>
        <n-form-item-grid-item :span="12" label="状态" path="userStatus">
          <n-select v-model:value="formModel.user_status" :options="userStatusOptions" />
        </n-form-item-grid-item>
      </n-grid>
      <n-space class="w-full pt-16px" :size="24" justify="end">
        <n-button class="w-72px" @click="closeModal">取消</n-button>
        <n-button class="w-72px" type="primary" @click="handleSubmit">确定</n-button>
      </n-space>
    </n-form>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, computed, reactive, watch } from 'vue';
import type { FormInst, FormItemRule } from 'naive-ui';
import { formRules, createRequiredFormRule } from '@/utils';
import { userStatusOptions, authorityOptions } from '@/constants';
import { fetchUpdateUser } from '~/src/service';

export interface Props {
  /** 弹窗可见性 */
  visible: boolean;
  /** 编辑的表格行数据 */
  editData?: UserManagement.User | null;
}

defineOptions({ name: 'TableActionModal' });

const props = withDefaults(defineProps<Props>(), {
  editData: null
});

interface Emits {
  (e: 'update:visible', visible: boolean): void;
}

const emit = defineEmits<Emits>();

const modalVisible = computed({
  get() {
    return props.visible;
  },
  set(visible) {
    emit('update:visible', visible);
  }
});
const closeModal = () => {
  modalVisible.value = false;
};

const title = computed(() => {
  return '编辑用户';
});

const formRef = ref<HTMLElement & FormInst>();

const formModel = reactive<Auth.FormModel>(createDefaultFormModel());

const rules: Record<keyof Auth.FormModel, FormItemRule | FormItemRule[]> = {
  user_name: createRequiredFormRule('请输入用户名'),
  authority: createRequiredFormRule('选择用户权限'),
  email: formRules.email,
  user_status: createRequiredFormRule('请选择用户状态')
};

function createDefaultFormModel(): Auth.FormModel {
  return {
    user_name: '',
    authority: 'user',
    email: null,
    user_status: 1
  };
}

async function handleUpdateFormModel() {
  if (props.editData) {
    Object.assign(props.editData, formModel);
    const { error } = await fetchUpdateUser(props.editData.user_id, props.editData);
    if (!error) {
      window.$message?.success('更新成功!');
      return true;
    }
  }
  window.$message?.error('更新失败!');
  return false;
}

async function handleSubmit() {
  await formRef.value?.validate();
  await handleUpdateFormModel();
  closeModal();
}

watch(
  () => props.visible,
  newValue => {
    if (newValue) {
      if (props.editData) {
        Object.assign(formModel, props.editData);
      }
    }
  }
);
</script>

<style scoped></style>
