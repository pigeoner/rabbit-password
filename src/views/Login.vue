<template>
  <div class="login-box">
    <el-form>
      <el-form-item>
        <el-input size="large" v-model="mainPwd" :placeholder="localMainPwd ? '请输入主密码' : '请确定一个主密码'" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" size="large" @click="login" :style="{ width: '100%' }">{{ localMainPwd ? '登录' : '确定' }}</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import CryptoJS from 'crypto-js';

const router = useRouter();

// 主密码
const mainPwd = ref('');

// 尝试获取主密码
const localMainPwd = localStorage.getItem('mainPassword');

const login = () => {
  // 先加密
  const encryptedData = CryptoJS.SHA512(mainPwd.value);
  const encryptedDataHex = encryptedData.toString(CryptoJS.enc.Hex);
  if (localMainPwd) {
    /* 如果没有设置主密码 */
    // 密码输入错误
    if (encryptedDataHex !== localMainPwd) return ElMessage.error('主密码错误');
    else ElMessage.success('登录成功');
  } else {
    /* 设置了主密码 */
    // 主密码长度不能小于 6
    if (mainPwd.value?.length >= 6) {
      ElMessage.success('主密码设置成功');
      localStorage.setItem('mainPassword', encryptedDataHex);
    } else return ElMessage.error('主密码长度不能小于6位');
  }
  // 设置登录状态
  sessionStorage.setItem('isLogin', true);
  router.push('/home');
};
</script>

<style lang="scss">
.login-box {
  position: fixed;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  .el-form {
    width: 40%;
  }
}
</style>
