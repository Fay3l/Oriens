import { createRouter, createWebHistory } from 'vue-router'
import { jwtDecode } from "jwt-decode";
import HomeView from '../views/HomeView.vue'
import Questionnaire from '@/views/survey/Questionnaire.vue'
import SignUpView from '@/views/auth/SignUpView.vue'
import LogInView from '@/views/auth/LogInView.vue'
import CallVideoView from '@/views/CallVideoView.vue'
import TheLayoutView from '@/views/layout/TheLayoutView.vue'
import DashboardView from '@/views/dashboard/DashboardView.vue'
import StartQuiz from '@/views/survey/StartQuiz.vue'
import EndQuiz from '@/views/survey/EndQuiz.vue'
import ResultQuiz from '@/views/survey/ResultQuiz.vue'
import ForgotPasswordView from '@/views/auth/ForgotPasswordView.vue';


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

        },
        {
          path: '/jobs',
          name: 'jobs',
          component: () => import('@/views/jobs/JobsView.vue'),
          meta: {
            requiresAuth: true
          }
        },
        {
          path: '/dashboard',
          name: 'dashboard',
          component: DashboardView,
          meta: {
            requiresAuth: true
          }
        },
        {
          path: '/start-quiz',
          name: 'start-quiz',
          component: StartQuiz,
        },
        {
          path: '/result-quiz',
          name: 'result-quiz',
          component: ResultQuiz
        }
      ]
    },
    {
      path: '/call',
      name: 'call',
      component: CallVideoView,
      meta: {
        requiresAuth: true
      }
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
          component: LogInView,
        },
        {
          name:'forgot-password',
          path: 'forgot-password',
          component: ForgotPasswordView
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
      path: '/survey',
      name: 'survey',
      component: Questionnaire,
      meta: {
        requiresAuth: true
      }
    }
  ],
});

router.beforeEach((to, from, next) => {
  if (to.matched.some(record => record.meta.requiresAuth)) {
    const token = localStorage.getItem('token');
    console.log('Token:', token);
    if (token) {
      try {
        const decoded: { exp?: number } = jwtDecode(token);
        const now = Math.floor(Date.now() / 1000);
        if (decoded.exp && decoded.exp > now) {
          next();
        } else {
          localStorage.removeItem('token');
          next('/auth/login');
        }
      } catch (e) {
        console.error('JWT decode error:', e);
        localStorage.removeItem('token');
        next('/auth/login');
      }
    } else {
      next('/auth/login');
    }
  } else {
    next();
  }
});

export default router
