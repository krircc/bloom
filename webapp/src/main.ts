/* tslint:disable */
import Vue from 'vue';
import * as Sentry from '@sentry/browser';
import * as Integrations from '@sentry/integrations';
import Vuetify from 'vuetify';
const { log, Level } = require('@bloom42/astro');

import 'vuetify/dist/vuetify.min.css';
import '@mdi/font/css/materialdesignicons.css';

import App from '@/App.vue';
import router from '@/bloom/kernel/router';
import store from '@/store';
import api from '@/bloom/kernel/api';
import filters from '@/bloom/kernel/filters';

import VuetifyToast from '@/bloom/kernel/components/Toast';
Vue.use(VuetifyToast);
declare module 'vue/types/vue' {
  interface Vue {
    $toast: any;
  }
}


// kernel
import Toolbar from '@/bloom/kernel/components/Toolbar.vue';
import Footer from '@/bloom/kernel/components/Footer.vue';

Vue.component('blm-toolbar', Toolbar);
Vue.component('blm-footer', Footer);

// Layouts
import DefaultLayout from '@/bloom/kernel/layouts/Default.vue';
import AuthLayout from '@/bloom/kernel/layouts/Auth.vue';
import UnauthenticatedLayout from '@/bloom/kernel/layouts/Unauthenticated.vue';
import AuthenticatedLayout from '@/bloom/kernel/layouts/Authenticated.vue';

Vue.component('blm-layout-default', DefaultLayout);
Vue.component('blm-layout-auth', AuthLayout);
Vue.component('blm-layout-authenticated', AuthenticatedLayout);
Vue.component('blm-layout-unauthenticated', UnauthenticatedLayout);

// import './registerServiceWorker';


// init sentry for bug tracking
Sentry.init({
  dsn: process.env.VUE_APP_SENTRY_URL,
  environment: process.env.NODE_ENV,
  integrations: [new Integrations.Vue({ Vue })],
});

// Check environement
[
  'NODE_ENV',
  'VUE_APP_SENTRY_URL',
  'VUE_APP_ROOT_DOMAIN',
].forEach((env_var) => {
  if (!env_var) {
    throw new Error(`Missing environment variable: ${env_var}`);
  }
});

// init stage dependant stuff
if (process.env.NODE_ENV === 'development') {
  Vue.config.productionTip = true;
} else {
  Vue.config.productionTip = false;

  if (process.env.NODE_ENV === 'production') {
    log.config({ level: Level.INFO });
  }
}

log.with({ env: process.env }).debug('env loaded');

// init libraries and components
Vue.use(Vuetify, {
  iconfont: 'mdi',
  icons: {
    loading: 'mdi-loading',
  },
});


Vue.use(filters);
api.init();

new Vue({
  render: (h: any) => h(App),
  router,
  store,
}).$mount('#app');