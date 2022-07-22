import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import router from './router'
import VueRouter from "vue-router";
// import * as io from 'socket.io-client'
// import VueSocketIO from 'vue-socket.io';

// Vue.use(new VueSocketIO({
//   debug: true,
//   connection: 'http://localhost:8000',
//   options: { path: "/api/events/" }
// }));

// import socketio from 'socket.io-client';
// import VueSocketIO from 'vue-socket.io';
// import { io } from "socket.io-client";

// io.connect('http://localhost:8000', {path: '/api/events'});


Vue.use(VueRouter);

Vue.config.productionTip = false

new Vue({
  vuetify,
  router,
  render: h => h(App)
}).$mount('#app')
