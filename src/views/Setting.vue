<template>
  <div class="title">
    <el-icon><Setting /></el-icon>
    设置
  </div>
  <!-- 修改密码的表单 -->
  <el-form :model="form" :rules="rules" label-width="80px">
    <el-form-item class="password-input">
      <el-form-item label="旧密码" prop="oldPassword">
        <el-input v-model="form.oldPassword" show-password />
      </el-form-item>
      <el-form-item label="新密码" prop="newPassword">
        <el-input v-model="form.newPassword" show-password />
      </el-form-item>
      <el-form-item label="确认密码" prop="confirmPassword">
        <el-input v-model="form.confirmPassword" show-password />
      </el-form-item>
    </el-form-item>
    <el-form-item class="submit">
      <el-button type="primary" @click="submitForm('form')">提交</el-button>
      <el-button @click="resetForm('form')">重置</el-button>
    </el-form-item>
  </el-form>
</template>

<script setup>
import { ref, reactive } from 'vue';
import { Setting } from '@element-plus/icons-vue';
const form = reactive({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
});

const rules = ref({
  oldPassword: [
    { required: true, message: '请输入旧密码', trigger: 'blur' },
    { min: 6, max: 20, message: '长度在 6 到 20 个字符', trigger: 'blur' }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, max: 20, message: '长度在 6 到 20 个字符', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认密码', trigger: 'blur' },
    { min: 6, max: 20, message: '长度在 6 到 20 个字符', trigger: 'blur' }
  ]
});
const submitForm = formName => {
  console.log(formName);
};
const resetForm = formName => {
  console.log(formName);
};
</script>

<style lang="scss" scoped>
.el-form {
  .el-form-item {
    border: none;
    .el-input {
      // box-shadow: 1px 2px 2px rgba(0, 0, 0, 0.2);
      :deep(.el-input__inner) {
        // box-shadow: none !important;
      }
      :deep(.el-input__wrapper) {
        // box-shadow: none !important;
      }
    }
  }

  .password-input {
    gap: 20px;
    :deep(.el-form-item__content) {
      flex-direction: column;
      justify-content: space-between;
      align-items: flex-start;
      gap: 20px;
      margin: 0 !important;
    }
  }
  .submit {
    position: fixed;
    bottom: 0;
    right: 20px;
  }
}
.title {
  font-size: 20px;
  font-weight: 700;
  margin-bottom: 20px;
  text-align: left;
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 5px;
}
</style>
