import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import router from './router'
import VueRouter from "vue-router";
import VueSSE from 'vue-sse';
import store from './store'

Vue.use(VueSSE);
Vue.use(VueRouter);

Vue.config.productionTip = false

new Vue({
  vuetify,
  router,
  store,
  render: h => h(App)
}).$mount('#app')
