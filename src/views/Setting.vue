<template>
  <div class="title">
    <el-icon><Setting /></el-icon>
    设置
  </div>
  <!-- 修改密码的表单 -->
  <el-row>
    <el-col :span="6" class="label"><span>修改密码：</span></el-col>
    <el-col :span="18" class="content">
      <el-form :model="form" :rules="rules" label-width="80px" ref="formRef">
        <el-form-item class="password-input">
          <el-form-item label="当前密码" prop="oldPassword">
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
          <el-button type="primary" @click="changeMainPwd('form')">应用</el-button>
          <el-button @click="resetForm('form')">重置</el-button>
        </el-form-item>
      </el-form></el-col
    >
  </el-row>
</template>

<script setup>
import { ref, reactive } from 'vue';
import { Setting } from '@element-plus/icons-vue';
import enc from '@/utils/encrypt';
import invoker from '../utils/invoker';
const form = reactive({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
});

// 表单引用
const formRef = ref(null);

const oldPassword = ref(localStorage.getItem('mainPassword'));

// 表单验证规则
const rules = ref({
  oldPassword: [
    { required: true, message: '请输入当前密码', trigger: 'blur' },
    { min: 6, max: 20, message: '长度在 6 到 20 个字符', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (enc.encryptSHA512(value) !== oldPassword.value) {
          callback(new Error('当前密码错误'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, max: 20, message: '长度在 6 到 20 个字符', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认密码', trigger: 'blur' },
    { min: 6, max: 20, message: '长度在 6 到 20 个字符', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value !== form.newPassword) {
          callback(new Error('两次输入的密码不一致'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    }
  ]
});

// 提交表单
const changeMainPwd = formName => {
  // // 测试
  // const newMainPwd = enc.encryptSHA512(form.newPassword);
  // invoker('change_main_pwd', { newPwd: enc.encrypt(newMainPwd), oldPwd: enc.encrypt(form.oldPassword) }, _ => {
  //   ElMessage.success('修改成功');
  //   localStorage.setItem('mainPassword', newMainPwd);
  // });
  formRef.value.validate(valid => {
    if (valid) {
      // 表单验证通过，执行提交操作
      const newMainPwd = enc.encryptSHA512(form.newPassword);
      invoker('change_main_pwd', { newPwd: enc.encryptSHA512(newMainPwd), oldPwd: enc.encryptSHA512(form.oldPassword) }, _ => {
        ElMessage.success('修改成功');
        localStorage.setItem('mainPassword', newMainPwd);
      });
    } else {
      // 表单验证不通过，进行错误处理
      ElMessage.error('表单验证不通过');
    }
  });
};
const resetForm = formName => {
  console.log(formName);
};
</script>

<style lang="scss" scoped>
.el-form {
  .el-form-item {
    border: none;
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
.label {
  text-align: left;
  padding-left: 24px;
  span {
    line-height: 32px;
    vertical-align: middle;
    font-weight: 700;
  }
}
</style>
