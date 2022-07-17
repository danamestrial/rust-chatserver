import Vue from 'vue'
import VueRouter from 'vue-router'
// import axios from "axios"
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/about',
    name: 'about',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/AboutView.vue')
  }, 
  {
    path: '/login',
    name: 'login',
    component: LoginView
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

// router.beforeEach(async (to, from, next) => {
//   const response = await axios.get("/api/").catch((error) => {
//     if (error.response) {
//       console.warn("something went wrong");
//     }
//   });

//   console.log(response);
//   console.log(to);
//   console.log(from);
//   console.log(next);
// });

export default router;