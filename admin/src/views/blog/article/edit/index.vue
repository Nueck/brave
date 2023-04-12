<template>
  <n-space :vertical="true" :size="26" class="m-r-20px m-l-20px">
    <n-space justify="start" class="w-auto">
      <n-button type="primary" @click="handleToTab">返回</n-button>
    </n-space>

    <n-space vertical class="w-auto">
      <n-space vertical justify="space-between" class="w-auto">
        <n-input v-model:value="contentData.title" size="large" maxlength="30" show-count clearable placeholder="标题">
        </n-input>
        <n-input
          v-model:value="contentData.subtitle"
          size="large"
          type="textarea"
          maxlength="120"
          clearable
          show-count
          placeholder="副标题"
        >
        </n-input>

        <n-upload
          action="http://localhost:2078/api/upload/img"
          :default-file-list="fileList"
          list-type="image-card"
          :max="1"
          @finish="handleFinish"
          @before-upload="beforeUpload"
        >
          上传封面</n-upload
        >
        <n-radio-group v-model:value="radioValue" name="radiogroup">
          <n-space>
            <n-radio v-for="song in songs" :key="song.value" :value="song.value">
              {{ song.label }}
            </n-radio>
          </n-space>
        </n-radio-group>
      </n-space>
      <md-editor v-model="contentData.content" :on-html-changed="handleHtmlCode" :preview="false" class="w-auto" />
      <n-space class="w-auto" justify="end">
        <template v-if="status">
          <n-button type="primary" class="w-180px h-36px" @click="handleSaveData()">保存文章</n-button>
        </template>
        <template v-else>
          <n-button type="primary" class="w-180px h-36px" @click="article.updateData(contentData)">保存修改</n-button>
        </template>
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
import { useArticlesStore } from '~/src/store';

const article = useArticlesStore();
const route = useRoute();
const { routerPush } = useRouterPush();

const radioValue = ref('Markdown');
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
const fileList = ref<UploadFileInfo[]>([]);
const contentData = ref<Blog.ArticleEditData>({
  table_id: 0,
  title: '',
  tag: '',
  subtitle: '',
  img_url: '',
  content: '',
  html_content: ''
});

const status = ref(true);
if (route.hash) {
  const str = route.hash.replace('#', '');
  const id = Number(str);
  if (id) {
    contentData.value.table_id = id;
    fetchArticleEditData(id)
      .then(value => {
        const { data } = value;
        if (data) {
          contentData.value.content = data.content;
          contentData.value.title = data.title;
          contentData.value.subtitle = data.subtitle;
          contentData.value.img_url = data.img_url;
          contentData.value.tag = data.tag;

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

// 处理渲染好的html数据
async function handleHtmlCode(h: string) {
  contentData.value.html_content = h;
}

async function handleSaveData() {
  await article.saveData(contentData.value, route.fullPath);
}

// 获取服务器的数据
const handleFinish = ({ file, event }: { file: UploadFileInfo; event?: ProgressEvent }) => {
  const data = (event?.target as XMLHttpRequest).response;
  const json = JSON.parse(data);
  contentData.value.img_url = json.url;
  return file;
};
</script>

<style scoped></style>
