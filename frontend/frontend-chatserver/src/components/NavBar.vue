<template>
  <div class="home">

   <v-app-bar
    color = "light-blue lighten-3"
    app
    flat
    >
      <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
    </v-app-bar>

    <v-navigation-drawer
      v-model="drawer"
      class = "brown darken-3 overflow-y-auto"
      dark
      app
      temporary
      absolute
    >
      <h1 class="white--text text-h2 font-weight-black" to="/">
        Hello
        <v-btn text icon link to="/">
          <v-icon size="40" color="amber"> mdi-hand-wave </v-icon>
        </v-btn>
      </h1>
      <h2 class="white--text text-left overflow-auto pl-3 pb-2">
        {{ this.$store.state.username }}
      </h2>

      <v-divider></v-divider>

      <div>
        <h3 class="text-left white--text text-h4 font-weight-medium pl-3 pt-2">
          Groups
        </h3>
        <v-list flat>
          <v-list-item
            v-for="name in commu"
            :key="name"
            link
            @click="changeRoom(name)"
            router
            to="/chat"
          >
            <v-list-item-icon>
              <v-icon>mdi-brain</v-icon>
            </v-list-item-icon>

            <v-list-item-content>
              <v-list-item-title class="text-left">{{
                name
              }}</v-list-item-title>
            </v-list-item-content>
          </v-list-item>
        </v-list>
      </div>

      <v-btn text absolute right bottom color="red" @click="logout()">
        <span>Sign Out</span>
        <v-icon right>mdi-logout</v-icon>
      </v-btn>
    </v-navigation-drawer>
  </div>
</template>

<script>
import axios from "axios";
import store from "@/store";
export default {
  name: "HomeView",
  data: () => ({
    username: "airbus",
    drawer: true,
    popup: false,
    commu: store.state.rooms,
  }),
  methods: {
    async logout() {
      await axios.get("/api/logout");
      this.$router.go("/");
      await store.dispatch("resetinfo");
    },
    changeRoom(Name) {
      if (store.state.roomname != Name) {
        store.state.roomname = Name;
        console.log("changed room to", store.state.roomname);
      }
    },
  },
};
</script>
