<template>
  <div class="home">

    <v-navigation-drawer
      v-model="drawer"
      class = "brown darken-3 overflow-y-auto"
      dark
      app
    >
      <h1 class = "white--text text-h2 font-weight-black" to="/">Hello
      <v-btn flat icon link to="/">
      <v-icon size ="40" color="amber">
        mdi-hand-wave
      </v-icon>
      </v-btn>
      </h1>
      <h2 class = "white--text text-left overflow-auto pl-3 pb-2">{{ username }}</h2>
      
      <v-divider></v-divider>

      <v-list-group
          :value="true"
          no-action
          class = "pt-2"
        >
          <template v-slot:activator>
            <v-list-item-content>
              <v-list-item-title class = "text-left white--text text-h4 font-weight-medium">Friends</v-list-item-title>
            </v-list-item-content>
          </template>

        <v-list subheader>
        <v-list-item
          v-for="[name] in friends"
          :key="name"
        >
          <v-list-item-content>
            <v-list-item-title class = "text-left text-h6">{{ name }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        </v-list>
      </v-list-group>

      <v-divider></v-divider>

      <div>
      <h3 class = "text-left white--text text-h4 font-weight-medium pl-3 pt-2">
        Community</h3>
      <v-list flat>
        <v-list-item
          v-for="name in commu"
          :key="name"
          link
          @click="getName(name)"
          router to="/chat"
        >
          <v-list-item-icon>
            <v-icon>mdi-brain</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title class = "text-left">{{ name }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
      </div>

      <v-btn 
      flat 
      absolute
      right
      bottom 
      color = "red"
      @click="logout()">
        <span>Sign Out</span>
        <v-icon right>mdi-logout</v-icon>
      </v-btn>

    </v-navigation-drawer>
    
    <v-app-bar
    id="appbar"
    color = "light-blue lighten-3"
    flat
    app
    >
      <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
    <v-spacer></v-spacer>

    <v-btn
      elevation="2"
      fab
      icon
      class = "mr-1 mt-7 cyan ligthen-2"
      :open-on-hover="hover"
    ><v-icon>mdi-cog</v-icon>
    
    <v-dialog
    v-model="add">
      <v-card>
        <v-card-title>Invite friend</v-card-title>
        <v-spacer></v-spacer>
        <v-btn @click="popup = false">mdi-close</v-btn>
        <v-img
        lazy-src=""></v-img>
      </v-card>
    </v-dialog>
    </v-btn>
    </v-app-bar>
  </div>
</template>

<script>
import axios from "axios";
import store from "@/store";
export default {
    name: "HomeView",
    data: () => ({
        username: "airbus",
        drawer: null,
        popup: false,
        friends: [
            ["gloria"],
            ["nut"]
        ],
        commu: store.state.rooms
    }),
    methods: {
      async logout() {
        await axios.get("/api/logout");
        this.$router.go("/");
        await store.dispatch("resetinfo");
      },
      getName(Name) {
        store.state.roomname = Name;
        console.log(store.state.roomname);
      }
    }
}
</script>
