import Vue from 'vue'
import VueRouter from 'vue-router'
import axios from "axios"
import store from '@/store'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'
import RegisterView from '../views/RegisterView.vue'
import ChatView from '../views/ChatView.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
    meta: {
      title: "Home",
    },
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView,
    meta: {
      title: "Login",
    },
  },
  {
    path: '/register',
    name: 'register',
    component: RegisterView,
    meta: {
      title: "Register",
    },
},
{
    path:'/chat',
    name:'chat',
    component: ChatView,
    meta: {
      title: "Chat",
    },
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

router.beforeEach(async (to, from, next) => {
  const response = await axios.get("/api/whoami").catch((error) => {
    if (error.response) {
      console.warn("something went wrong");
    }
  });

  // Check prints
  // console.log(response.data);
  await store.dispatch("storedinfo", response.data);
  // console.log(store.state.username);
  // console.log(store.state.rooms);

  const isLoggedIn = store.state.status;
  if (to.name === "login" && isLoggedIn) {
    next("/");
  } else {
    next();
  }
});

export default router;