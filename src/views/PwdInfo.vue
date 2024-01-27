<template>
  <el-page-header :icon="DArrowLeft" @back="router.push('/home')">
    <template #title>
      <span class="back"> 返回 </span>
    </template>
    <template #content>
      <span class="title"> {{ title }} </span>
    </template>
  </el-page-header>
  <el-form :model="form" :rules="rules" label-width="120px" ref="formRef">
    <el-form-item label="服务名称" prop="name">
      <el-input v-model="form.name" :disabled="isDisabled" />
    </el-form-item>
    <el-form-item label="url 链接" prop="url">
      <el-input v-model="form.url" :disabled="isDisabled" />
    </el-form-item>
    <el-form-item label="描述信息" prop="description">
      <el-input v-model="form.description" type="textarea" :rows="3" :disabled="isDisabled" />
    </el-form-item>
    <el-form-item label="登录用户名" prop="username">
      <el-input v-model="form.username" :disabled="isDisabled" />
    </el-form-item>
    <el-form-item label="邮箱" prop="email">
      <el-input v-model="form.email" :disabled="isDisabled" />
    </el-form-item>
    <el-form-item label="手机号" prop="phone">
      <el-input v-model="form.phone" :disabled="isDisabled" />
    </el-form-item>
    <el-form-item label="密码" prop="pwd">
      <el-row style="width: 100%">
        <el-col :span="18"> <el-input v-model="form.pwd" :disabled="isDisabled" type="password" show-password /></el-col>
        <el-col :span="6" style="text-align: right">
          <el-button @click="generatePwd" :disabled="isDisabled">随机生成</el-button>
        </el-col>
      </el-row>
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
import { computed, reactive, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { DArrowLeft } from '@element-plus/icons-vue';
import invoker from '../utils/invoker';
import enc from '@/utils/encrypt';
import generateRandomPwd from '@/utils/randomPwd';
import { h } from 'vue';
const route = useRoute();
const router = useRouter();

// 主密码
const mainPwd = ref(localStorage.getItem('mainPassword') || '');

// 通过路由参数获得当前界面的密码 id
// 如果 id 为 undefined，说明是新增密码
const pwdId = ref(+route.params.id || 0);

// 是否为编辑模式
const isEdit = ref(false);

// 表单是否禁用
const isDisabled = computed(() => !isEdit.value && !!pwdId.value);

// 获取信息
!!pwdId.value &&
  invoker('getAccount', pwdId.value, data => {
    form.name = data.name;
    form.url = data.url;
    form.description = data.description;
    form.username = data.username;
    form.email = data.email;
    form.phone = data.phone;
    form.last_update = data.last_update;
    form.pwd = enc.decrypt(data.pwd, mainPwd.value);
  });

// 标题
const title = ref(pwdId.value ? '密码信息' : '添加密码');

const form = reactive({
  name: '', // 网站或服务名称
  url: null, // 链接，如果服务不是网站则为null
  description: '', // 描述信息
  username: '', // 登录用户名
  email: '', // 邮箱
  phone: '', // 手机号
  last_update: '', // 最后更新时间
  pwd: '' // 密码
});

// 表单引用
const formRef = ref(null);

// 表单验证规则
const rules = ref({
  name: [
    { required: true, message: '请输入服务名称', trigger: 'blur' },
    { min: 1, max: 255, message: '长度在 1 到 255 个字符', trigger: 'blur' }
  ],
  url: [
    { required: false, message: '请输入链接', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value && !/^https?:\/\/.*/.test(value)) {
          callback(new Error('链接格式不正确'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    }
  ],
  description: [
    { required: false, message: '请输入描述信息', trigger: 'blur' },
    { min: 0, max: 65535, message: '长度在 0 到 65535 个字符', trigger: 'blur' }
  ],
  // 用户名不确定是否需要做限制
  username: [{ required: true, message: '请输入登录用户名', trigger: 'blur' }],
  email: [
    { required: false, message: '请输入邮箱', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value && !/^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/.test(value)) {
          callback(new Error('邮箱格式不正确'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    }
  ],
  phone: [
    { required: false, message: '请输入手机号', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value && !/^1[3456789]\d{9}$/.test(value)) {
          callback(new Error('手机号格式不正确'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    }
  ],
  pwd: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 32, message: '长度在 6 到 32 个字符', trigger: 'blur' }
  ]
});

// 提交
const onSubmit = () => {
  Object.assign(form, { ...form, name: form.name.trim(), url: form.url?.trim(), description: form.description?.trim() });
  formRef.value.validate(valid => {
    if (valid) {
      // 表单验证通过，执行提交操作
      if (pwdId.value !== 0) {
        invoker(
          'updateAccount',
          { id: pwdId.value, ...form, pwd: enc.encrypt(form.pwd, mainPwd.value), last_update: new Date().toLocaleString() },
          _ => {
            ElMessage.success('修改成功');
            isEdit.value = false;
          }
        );
      } else {
        invoker('addAccount', { ...form, pwd: enc.encrypt(form.pwd, mainPwd.value), last_update: new Date().toLocaleString() }, _ => {
          ElMessage.success('创建成功');
          isEdit.value = false;
          router.push('/home');
        });
      }
    } else {
      // 表单验证不通过，进行错误处理
      ElMessage.error('表单验证不通过');
    }
  });
};

// 密码长度
const pwdLength = ref(16);

// 生成随机密码
const generatePwd = () => {
  ElMessageBox({
    title: '设置密码长度',
    message: () =>
      h('div', { style: { textAlign: 'center' } }, [
        h('p', { style: { margin: '10px 0 20px 0' } }, '密码最小长度6位，最大长度32位'),
        h(ElInputNumber, {
          modelValue: pwdLength.value,
          'onUpdate:modelValue': (val: boolean | string | number) => {
            pwdLength.value = val;
          },
          // 最小长度6位，最大长度32位
          min: 6,
          max: 32
        })
      ])
  })
    .then(() => {
      form.pwd = generateRandomPwd(pwdLength.value);
    })
    .catch(() => {});
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
