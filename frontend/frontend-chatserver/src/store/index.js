import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    username: null,
    status: false,
    roomname: null,
    rooms: [],
    storage: new Map(), //-> Map<roomname, messages>

  //   {
  //     room: "lobby", message: [
  //       {
  //         senderName: "gloria",
  //         role: "Support",
  //         text: "kik",
  //         timestamp: "12:23, 3 Jul 2022",
  //       },
  //       {
  //         senderName: "airbus",
  //         role: "Carry",
  //         text: "MK mai",
  //         timestamp: "12:30, 3 Jul 2022",
  //       },
  //       {
  //         senderName: "nut",
  //         role: "Support",
  //         text: "okk",
  //         timestamp: "12:35, 3 Jul 2022",
  //       }]
  //   },
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
    setStorage(state, rooms) {
      JSON.parse(rooms).forEach(element => {
        console.log(typeof element);
        if (!this.state.storage.has(element)) {
          this.state.storage.set(element, []);
        }
      });
      console.log(this.state.storage);
    },
    addRooms(state, room) {
      this.state.rooms.push(room);
    },
    addMessage(state, payload) {
      console.log(payload);
      if (this.state.storage.has(payload.room)) {
        this.state.storage.get(payload.room).push(payload.message);
      }
      console.log(this.state.storage);
      console.log(this.state.storage.get('lobby'));
    },
    // changeRoom(state, newroom) {
      
    // }
  },
  actions: {
    storedinfo({ commit }, response) {
      commit("setUsername", response.username);
      commit("setStatus", response.status);
      commit("setRooms", response.rooms);
      commit("setStorage", response.rooms);
    },
    resetinfo({ commit }) {
      commit("setStatus", false);
      commit("setUsername", null);
    },
  },
  modules: {
  }
})
