import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    username: null,
    status: false,
  },
  getters: {
  },
  mutations: {
    setUsername(state, username) {
      this.state.username = username;
    },
    setStatus(state, status) {
      this.state.status = status;
    },
  },
  actions: {
    storedinfo({ commit }, response) {
      commit("setUsername", response.username);
      commit("setStatus", response.status);
    },
    resetinfo({ commit }) {
      commit("setStatus", false);
      commit("setUsername", null);
    },
  },
  modules: {
  }
})
