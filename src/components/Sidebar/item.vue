<template>
  <div class="sidebar-item">
    <el-icon><component :is="icon" /></el-icon>
    <span>{{ name }}</span>
  </div>
</template>

<script setup>
defineProps({
  icon: {
    type: Object,
    default: null
  },
  name: {
    type: String,
    default: ''
  }
});
</script>

<style lang="scss" scoped>
.sidebar-item {
  color: #fff;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 15px 0;
  transition: all 0.3s;
}
</style>
