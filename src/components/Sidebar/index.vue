<template>
  <div class="sidebar">
    <div class="title">
      <div class="avatar">
        <img src="@/assets/rabbit.jpg" />
      </div>
      <h1>Rabbit Password</h1>
    </div>
    <div class="tabs">
      <router-link to="/home"><Item :icon="Lock" name="密码" /></router-link>
      <router-link to="/setting"><Item :icon="Setting" name="设置" /></router-link>
      <router-link to="/about"><Item :icon="InfoFilled" name="关于" /></router-link>
    </div>
  </div>
</template>

<script setup>
import Item from './item.vue';
import { Lock, Setting, InfoFilled } from '@element-plus/icons-vue';
</script>

<style lang="scss" scoped>
.title {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  margin-bottom: 20px;
  gap: 10px;
  .avatar {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    border: 6px solid rgba(133, 152, 187, 0.505);
    overflow: hidden;
    img {
      width: 100%;
      height: 100%;
    }
  }
  h1 {
    font-size: 20px;
    // font-weight: 400;
  }
}
.tabs {
  .router-link-active {
    height: 100%;
    width: 100%;
    .sidebar-item {
      // background-color: #868f98;
      background-color: #515d69;
    }
  }
  a:hover {
    .sidebar-item {
      // background-color: #b5bec7;
      background-color: #6e7780;
    }
  }
}
</style>
