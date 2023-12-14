<template>
  <el-page-header :icon="DArrowLeft" @back="router.push('/home')">
    <template #title>
      <span class="back"> 返回 </span>
    </template>
    <template #content>
      <span class="title"> {{ title }} </span>
    </template>
  </el-page-header>
  <el-form :model="form" label-width="120px">
    <el-form-item label="服务名称">
      <el-input v-model="form.name" />
    </el-form-item>
    <el-form-item label="url 链接">
      <el-input v-model="form.url" />
    </el-form-item>
    <el-form-item label="描述信息">
      <el-input v-model="form.description" type="textarea" :rows="3" />
    </el-form-item>
    <el-form-item label="登录用户名">
      <el-input v-model="form.username" />
    </el-form-item>
    <el-form-item label="邮箱">
      <el-input v-model="form.email" />
    </el-form-item>
    <el-form-item label="手机号">
      <el-input v-model="form.phone" />
    </el-form-item>
    <el-form-item label="密码">
      <el-input v-model="form.pwd" />
    </el-form-item>
    <el-form-item class="submit-buttons">
      <template v-if="!pwdId">
        <el-button type="success" @click="onSubmit">添加</el-button>
      </template>
      <template v-else-if="isEdit">
        <el-button type="warning" @click="onSubmit">应用</el-button>
        <el-button @click="isEdit = false">取消</el-button>
      </template>
      <template v-else>
        <el-button type="primary" @click="isEdit = true">编辑</el-button>
      </template>
    </el-form-item>
  </el-form>
</template>

<script lang="ts" setup>
import { reactive, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { DArrowLeft } from '@element-plus/icons-vue';
const route = useRoute();
const router = useRouter();

// 通过路由参数获得当前界面的密码 id
// 如果 id 为 undefined，说明是新增密码
const pwdId = ref(route.params.id);

// 标题
const title = ref(pwdId.value ? '密码信息' : '添加密码');

// 是否为编辑模式
const isEdit = ref(false);

const form = reactive({
  name: '', // 网站或服务名称
  url: null, // 链接，如果服务不是网站则为null
  description: '', // 描述信息
  username: '', // 登录用户名
  email: '', // 邮箱
  phone: '', // 手机号
  lastUpdate: '', // 最后更新时间
  pwd: '' // 密码
});

// 提交
const onSubmit = () => {
  console.log('submit!');
};
</script>
<style lang="scss" scoped>
.el-page-header {
  margin-bottom: 30px;
  .title,
  .back {
    font-weight: 700;
    color: #555;
  }
  .back {
    cursor: pointer;
    color: #999;
  }
}
.el-form {
  transform: translate(-25px, 0);
  margin: 0 30px;
  .submit-buttons {
    position: fixed;
    bottom: -80px;
    right: 0;
  }
}
</style>
