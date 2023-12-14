<template>
  <div>
    <h1>Test</h1>
    <el-button @click="test">test</el-button>
  </div>
</template>

<script setup>
const test = () => {
  console.log('test');
};
</script>

<style lang="scss" scoped></style>
