<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from './components/Greet.vue';
import SideBar from './components/SideBar/index.vue';
</script>

<template>
  <el-container class="container">
    <el-aside width="150px">
      <SideBar />
    </el-aside>
    <el-main>
      <router-view></router-view>
    </el-main>
  </el-container>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
.container {
  padding: 0;
  height: 100vh;
  .el-aside {
    /* background-color: #545c64; */
    background-color: #2c3034;
    color: #e3e4e4;
  }
  .el-main {
    position: relative;
    color: #606266;
  }
}
</style>
