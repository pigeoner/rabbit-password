import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '',
      redirect: '/home'
    },
    {
      path: '/home',
      name: 'home',
      component: () => import('../views/Home.vue')
    },
    {
      path: '/setting',
      name: 'setting',
      component: () => import('../views/Setting.vue')
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/About.vue')
    },
    {
      path: '/pwd/:id?',
      name: 'pwdInfo',
      component: () => import('../views/PwdInfo.vue')
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/Login.vue')
    }
  ]
});

// beforeEach
router.beforeEach((to, from, next) => {
  if (sessionStorage.getItem('isLogin') || to.path === '/login') {
    next();
  } else {
    next('/login');
  }
});

export default router;
