<template>
  <div class="header">
    <el-input v-model="searchContent" class="w-50 m-2" placeholder="搜索密码" :prefix-icon="Search" />
    <el-button class="m-2 search-button" @click="searchPwd">
      <el-icon><Search /></el-icon>搜索
    </el-button>
    <el-button type="primary" @click="router.push({ name: 'pwdInfo' })">
      <el-icon><Plus /></el-icon>添加
    </el-button>
  </div>
  <div class="list-title">
    <span v-for="item in listTitle">{{ item }}</span>
  </div>
  <div class="pwd-list">
    <el-card
      shadow="hover"
      class="pwd-list-item"
      :body-style="{
        padding: '8px',
        display: 'flex',
        width: '100%'
      }"
      v-for="pwd in passwords"
    >
      <template #default>
        <div class="content">
          <el-tooltip class="item" effect="dark" :content="pwd.url" placement="top-start" :disabled="!pwd.url">
            <a :href="pwd.url" target="_blank" class="favicon">
              <img :src="getFavicon(pwd.url)" />
            </a>
          </el-tooltip>
          <div class="title">
            <h4>{{ pwd.name }}</h4>
            <p class="description">{{ pwd.description }}</p>
          </div>
          <div class="user">
            <el-icon><User /></el-icon>
            <span>{{ pwd.username }}</span>
          </div>
          <div class="last-update">
            <span>{{ pwd.last_update.split(' ')[0] }}</span>
          </div>
          <div class="operation">
            <el-button text :icon="CopyDocument" @click="copyPwd(pwd.pwd)" />
            <el-button text :icon="ZoomIn" @click="showInfo(pwd.id)" />
            <el-button text :icon="Delete" @click="deletePwd(pwd.id)" />
          </div>
        </div>
      </template>
    </el-card>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { Search, Plus, User, Delete, ZoomIn, CopyDocument } from '@element-plus/icons-vue';
import { writeText } from '@tauri-apps/api/clipboard';
import rabbit from '@/assets/rabbit.jpg';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/tauri';
const router = useRouter();

// 表头
const listTitle = ['名称', '用户名', '最后更新时间', '操作'];

// 根据 url 获取图标
const getFavicon = url => {
  // 截取第三个斜杠之前的内容
  // e.g. https://www.baidu.com
  if (!url) return rabbit;
  const reg = /^https?:\/\/[^/]+/;
  const res = reg.exec(url);
  return `${res}/favicon.ico`;
};

// 密码列表
const passwords = ref([]);
invoke('query').then(res => {
  const { code, msg, data } = res;
  if (code === 0) {
    passwords.value = data;
    passwords.value.map(item => {
      item.favicon = item.url ? getFavicon(item.url) : null;
    });
  } else if (code === -1) {
    ElMessage.error(msg);
  }
});

// 复制密码
const copyPwd = async pwd => {
  writeText(pwd)
    .then(res => {
      ElMessage.success('复制成功');
    })
    .catch(err => {
      ElMessage.error('复制失败');
    });
};

// 搜索
const searchContent = ref('');
const searchPwd = () => {
  invoke('query', { queryContent: searchContent.value }).then(res => {
    const { code, msg, data } = res;
    if (code === 0) {
      passwords.value = data;
      passwords.value.map(item => {
        item.favicon = item.url ? getFavicon(item.url) : null;
      });
    } else if (code === -1) {
      ElMessage.error(msg);
    }
  });
};

// 删除密码
const deletePwd = async id => {};

// 跳转到密码详情页
const showInfo = id => {
  router.push({
    name: 'pwdInfo',
    params: {
      id
    }
  });
};
</script>

<style lang="scss" scoped>
.header {
  display: flex;
  padding-bottom: 30px;
  border-bottom: 1px solid #dbdbdb;
  .search-button {
    margin-left: 12px;
    :deep(.el-input__inner) {
      border-radius: 0;
    }
  }
  .el-icon {
    margin-right: 6px;
  }
}
.list-title {
  display: flex;
  justify-content: space-between;
  padding: 0 8px;
  margin-top: 15px;
  span {
    display: inline-block;
    font-size: 14px;
    font-weight: 500;
    margin: 0 auto;
    text-align: left;
    font-weight: 900;
    color: #999;
    &:first-child {
      width: 210px;
      margin: 0;
      transform: translate(60px, 0);
    }
    &:nth-child(3) {
      transform: translate(-5%, 0);
    }
    &:last-child {
      margin-right: 32px;
    }
  }
}
.pwd-list {
  padding-top: 15px;
  display: flex;
  flex-wrap: wrap;
  flex-direction: column;
  gap: 10px;
  &-item {
    display: flex;
    justify-content: flex-start;
    width: 100%;
    height: 60px;
    background-color: #fff;
    border-radius: 4px;
    .content {
      display: flex;
      flex-direction: 'row';
      justify-content: flex-start;
      text-align: left;
      width: 100%;
      .favicon {
        height: 100%;
        width: calc(60px - 16px);
        // border-radius: 4px 0 0 4px;
        overflow: hidden;
        margin-right: 10px;
        img {
          width: 100%;
          height: 100%;
        }
      }
      .title {
        h1,
        h2,
        h3,
        h4,
        .description {
          overflow: hidden;
          width: 150px;
          text-overflow: ellipsis;
          white-space: nowrap;
        }
        .description {
          font-size: 12px;
          color: #999;
        }
      }
      .user {
        display: flex;
        align-items: center;
        gap: 4px;
        margin: 0 auto;
      }
      .last-update,
      .operation {
        display: flex;
        justify-content: center;
        align-items: center;
        margin: 0 auto;
        .el-button {
          padding: 5px;
        }
        .el-button + .el-button {
          margin-left: 6px;
        }
      }
      .user,
      .last-update {
        color: #666;
        font-size: 15px;
      }
      .operation {
        margin-right: 5px;
      }
    }
    // box-shadow: 1px 2px 2px rgba(0, 0, 0, 0.2);
    // margin-bottom: 20px;
  }
}
</style>
