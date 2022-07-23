import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    username: null,
    status: false,
    roomname: null,
    rooms: null,
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
    setRooms(state, roomname) {
      this.state.rooms = JSON.parse(roomname);
    },
    addRooms(state, room) {
      this.state.rooms.push(room);
    }
  },
  actions: {
    storedinfo({ commit }, response) {
      commit("setUsername", response.username);
      commit("setStatus", response.status);
      commit("setRooms", response.rooms);
    },
    resetinfo({ commit }) {
      commit("setStatus", false);
      commit("setUsername", null);
    },
  },
  modules: {
  }
})
