import './base.css';

import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import { VueQueryPlugin } from '@tanstack/vue-query';

import App from './App.vue';

// Authentication pages
import LoginPage from './pages/LoginPage.vue';
import FirstLoginPage from './pages/FirstLoginPage.vue';

// Admin pages
import AdminPage from './pages/admin/AdminPage.vue';
import OverviewPage from './pages/admin/OverviewPage.vue';

const router = createRouter({
  history: createWebHistory('/'),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: LoginPage,
    },
    {
      path: '/fl',
      name: 'first-login',
      component: FirstLoginPage,
    },
    {
      path: '/admin',
      component: AdminPage,
      children: [
        { path: '', name: 'overview', component: OverviewPage, meta: { adminTitle: 'Overview' } },
      ],
    },
  ],
});

createApp(App).use(VueQueryPlugin).use(router).mount('#app');
