<template>
  <n-space :vertical="true" :size="26" class="m-r-20px m-l-20px">
    <n-space justify="start" class="w-auto">
      <n-button type="primary" @click="handleToTab">返回</n-button>
    </n-space>

    <n-space vertical class="w-auto">
      <n-space vertical justify="space-between" class="w-auto">
        <n-input v-model:value="title" size="large" maxlength="30" show-count clearable placeholder="标题"> </n-input>
        <n-input
          v-model:value="subtitle"
          size="large"
          type="textarea"
          maxlength="120"
          clearable
          show-count
          placeholder="副标题"
        >
        </n-input>

        <n-upload
          action="__HTTP__://www.mocky.io/v2/5e4bafc63100007100d8b70f"
          :default-file-list="fileList"
          list-type="image-card"
          :max="1"
          @before-upload="beforeUpload"
        >
          上传封面</n-upload
        >
        <!-- <n-modal v-model:show="showModal" preset="card" style="width: 600px" title="一张很酷的图片">
          <img :src="previewImageUrl" style="width: 100%" />
        </n-modal> -->
        <n-radio-group v-model:value="radioValue" name="radiogroup">
          <n-space>
            <n-radio v-for="song in songs" :key="song.value" :value="song.value">
              {{ song.label }}
            </n-radio>
          </n-space>
        </n-radio-group>
      </n-space>
      <md-editor v-model="text" class="w-auto" />
      <n-space class="w-auto" justify="end">
        <n-button type="primary" class="w-180px h-36px" @click="handleToTab">
          <template v-if="status">保存</template>
          <template v-else>保存编辑</template>
        </n-button>
      </n-space>
    </n-space>
  </n-space>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import type { UploadFileInfo } from 'naive-ui';
import MdEditor from 'md-editor-v3';
import { routeName } from '@/router';
import { useRouterPush } from '@/composables';
import { fetchArticleEditData } from '~/src/service/api/article';
import 'md-editor-v3/lib/style.css';

const route = useRoute();
const { routerPush } = useRouterPush();

const text = ref('');
const title = ref('');
const subtitle = ref('');
const radioValue = ref('Markdown');
const fileList = ref<UploadFileInfo[]>([]);

const status = ref(true);
if (route.query.tableId) {
  const id = Number(route.query.tableId);
  if (id) {
    fetchArticleEditData(id)
      .then(value => {
        const { data } = value;
        if (data) {
          text.value = data.content;
          title.value = data.title;
          subtitle.value = data.subtitle;

          fileList.value.push({
            id: 'a',
            name: 'cover',
            status: 'finished',
            url: data.img_url
          });

          status.value = false;
        }
      })
      .catch(() => {
        window.$message?.error('页面数据获取失败');
      });
  }
}

const songs = [
  {
    value: 'Markdown',
    label: 'Markdown'
  },
  {
    value: 'Rich text',
    label: '富文本'
  }
];

function handleToTab() {
  routerPush({ name: routeName('blog_article_table') });
}

async function beforeUpload(data: { file: UploadFileInfo; fileList: UploadFileInfo[] }) {
  if (
    data.file.file?.type === 'image/jpg' ||
    data.file.file?.type === 'image/png' ||
    data.file.file?.type === 'image/jpeg'
  ) {
    return true;
  }
  window.$message?.error('只能上传jpeg/jpg/png格式的图片文件,请重新上传');
  return false;
}
</script>

<style scoped></style>
