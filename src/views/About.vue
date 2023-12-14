<template>
  <div class="title">
    <div class="avatar">
      <img src="@/assets/rabbit.jpg" alt="" />
    </div>
    <!-- <h1>啥也不是，净唠比嗑</h1> -->
    <h1>Rabbit Password</h1>
  </div>
  <div class="content">
    <p class="introduction"><span>啥也不是，净唠比嗑</span></p>
    <p class="text">一个密码管理器，目前仍处于开发阶段。</p>
  </div>
  <footer>
    <p>作者: <a>pigeoner</a> &lt;zcpigeoner@163.com&gt;</p>
    <p>版本: {{ appVersion }}</p>
    <p>许可: MIT License</p>
    <p>仓库: <a href="https://github.com/pigeoner/rabbit-password">github.com/pigeoner/rabbit-password</a></p>
  </footer>
</template>

<script setup>
import { getVersion } from '@tauri-apps/api/app';
import { ref, onMounted } from 'vue';
const appVersion = ref('');

onMounted(() => {
  getVersion().then(version => {
    appVersion.value = version;
  });
});
</script>

<style lang="scss" scoped>
.title {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 20px;
  .avatar {
    width: 36px;
    height: 36px;
    // border-radius: 50%;
    overflow: hidden;
    img {
      width: 100%;
      height: 100%;
    }
  }
  h1 {
    display: inline-block;
    font-size: 30px;
  }
}
.content {
  text-align: left;
  .introduction {
    margin: 20px 0;
    background-color: #e2e5e5;
    border-left: 8px solid #9498a78d;
    padding: 6px 0 6px 52px;
    span {
      display: inline-block;
      transform: skewX(-10deg);
    }
  }
  .text {
    padding-left: 10px;
    font-size: 16px;
    line-height: 1.5;
  }
}
footer {
  position: absolute;
  left: 30px;
  bottom: 30px;
  text-align: left;
  a {
    color: inherit;
    text-decoration: underline;
  }
}
</style>
