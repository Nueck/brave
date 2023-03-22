<template>
  <n-card title="用户管理" :bordered="false" class="rounded-16px shadow-sm">
    <n-space class="pb-12px" justify="space-between">
      <n-space>
        <n-button type="primary" @click="handleAddTable">
          <icon-ic-round-plus class="mr-4px text-20px" />
          新增
        </n-button>
        <!-- <n-button type="error">
          <icon-ic-round-delete class="mr-4px text-20px" />
          删除
        </n-button> -->
      </n-space>
      <n-space align="center" :size="15">
        <n-button size="small" type="primary" @click="getTableData">
          <icon-mdi-refresh :class="{ 'animate-spin': loading }" />
        </n-button>
      </n-space>
    </n-space>
    <n-data-table :columns="columns" :data="tableData" :loading="loading" :pagination="pagination" />
    <table-action-modal v-model:visible="visible" :type="modalType" :edit-data="editData" />
  </n-card>
</template>

<script setup lang="tsx">
import { reactive, ref } from 'vue';
import type { Ref } from 'vue';
import { NButton, NPopconfirm, NSpace, NTag } from 'naive-ui';
import type { DataTableColumns, PaginationProps } from 'naive-ui';
import { fetchUserList } from '@/service';
import { useBoolean, useLoading } from '@/hooks';
import { userAuthority, userStatusLabels } from '@/constants';
import TableActionModal from './components/TableActionModal.vue';
import type { ModalType } from './components/TableActionModal.vue';

const { loading, startLoading, endLoading } = useLoading(false);
const { bool: visible, setTrue: openModal } = useBoolean();

const tableData = ref<UserManagement.User[]>([]);
function setTableData(data: UserManagement.User[]) {
  tableData.value = data;
}

async function getTableData() {
  startLoading();
  const { data } = await fetchUserList();
  if (data) {
    setTimeout(() => {
      setTableData(data);
      endLoading();
    }, 1000);
  }
}

const columns: Ref<DataTableColumns<UserManagement.User>> = ref([
  {
    type: 'selection',
    align: 'center'
  },
  {
    key: 'index',
    title: '序号',
    align: 'center',
    sorter: (row1, row2) => row1.index - row2.index
  },
  {
    key: 'userName',
    title: '用户名',
    align: 'center',
    defaultSortOrder: 'ascend',
    sorter: 'default'
  },
  {
    key: 'authority',
    title: '权限',
    align: 'center',
    render: row => {
      if (row.authority) {
        const tagTypes: Record<UserManagement.UserAuthority, NaiveUI.ThemeColor> = {
          admin: 'warning',
          super: 'error',
          user: 'success'
        };

        return <NTag type={tagTypes[row.authority]}>{userAuthority[row.authority]}</NTag>;
      }

      return <span></span>;
    },
    defaultFilterOptionValues: [],
    filterOptions: [
      {
        label: '超级管理员',
        value: 'super'
      },
      {
        label: '管理员',
        value: 'admin'
      },
      {
        label: '用户',
        value: 'user'
      }
    ],
    filter(value: string, row) {
      // eslint-disable-next-line no-implicit-coercion, no-bitwise
      return Boolean(~row.authority.indexOf(value));
    }
  },
  {
    key: 'phone',
    title: '手机号码',
    align: 'center'
  },
  {
    key: 'email',
    title: '邮箱',
    align: 'center'
  },
  {
    key: 'userStatus',
    title: '状态',
    align: 'center',
    render: row => {
      if (row.userStatus) {
        const tagTypes: Record<UserManagement.UserStatusKey, NaiveUI.ThemeColor> = {
          '1': 'success',
          '2': 'error',
          '3': 'warning',
          '4': 'default'
        };

        return <NTag type={tagTypes[row.userStatus]}>{userStatusLabels[row.userStatus]}</NTag>;
      }
      return <span></span>;
    }
  },
  {
    key: 'actions',
    title: '操作',
    align: 'center',
    render: row => {
      return (
        <NSpace justify={'center'}>
          <NButton type="primary" ghost size={'small'} onClick={() => handleEditTable(row.id)}>
            编辑
          </NButton>
          <NPopconfirm onPositiveClick={() => handleDeleteTable(row.id)}>
            {{
              default: () => '确认删除',
              trigger: () => (
                <NButton type="primary" ghost size={'small'}>
                  删除
                </NButton>
              )
            }}
          </NPopconfirm>
        </NSpace>
      );
    }
  }
]) as Ref<DataTableColumns<UserManagement.User>>;

const modalType = ref<ModalType>('add');

function setModalType(type: ModalType) {
  modalType.value = type;
}

const editData = ref<UserManagement.User | null>(null);

function setEditData(data: UserManagement.User | null) {
  editData.value = data;
}

function handleAddTable() {
  openModal();
  setModalType('add');
}

function handleEditTable(rowId: string) {
  const findItem = tableData.value.find(item => item.id === rowId);
  if (findItem) {
    setEditData(findItem);
  }
  setModalType('edit');
  openModal();
}

function handleDeleteTable(rowId: string) {
  window.$message?.info(`点击了删除，rowId为${rowId}`);
}

const pagination: PaginationProps = reactive({
  page: 1,
  pageSize: 10,
  onChange: (page: number) => {
    pagination.page = page;
  },
  onUpdatePageSize: (pageSize: number) => {
    pagination.pageSize = pageSize;
    pagination.page = 1;
  }
});

function init() {
  getTableData();
}

// 初始化
init();
</script>

<style scoped></style>
