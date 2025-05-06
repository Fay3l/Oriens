import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import Questionnaire from '@/components/Questionnaire.vue'
import SignUpView from '@/views/auth/SignUpView.vue'
import LogInView from '@/views/auth/LogInView.vue'
import CallVideoView from '@/views/CallVideoView.vue'
import TheLayoutView from '@/views/layout/TheLayoutView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'layout',
      component: TheLayoutView,
      children: [
        {
          path: '',
          name: 'home',
          component: HomeView,
        }
      ]
    },
    {
      path:'/call',
      name: 'call',
      component: CallVideoView
    },
    {
      path: '/auth',
      name: 'auth',
      children: [
        {
          name: 'signup',
          path: 'signup',
          component: SignUpView,
        },
        {
          name: 'login',
          path: 'login',
          component: LogInView ,
        }
      ]
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
    {
      path:'/survey',
      name: 'survey',
      component: Questionnaire,
    }
  ],
});

router.beforeEach((to, from, next) => {
  if (to.meta.requiresAuth) {
    const token = localStorage.getItem('token');
    if (token) {
      // User is authenticated, proceed to the route
      next();
    } else {
      // User is not authenticated, redirect to login
      next('/login');
    }
  } else {
    // Non-protected route, allow access
    next();
  }
});

export default router
