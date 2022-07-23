<template>
<div class="home">
      <NavBar/>
      <v-app-bar
    id="appbar"
    color = "light-blue lighten-3"
    flat
    app
    >
      <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
    </v-app-bar>
    <v-main>
      <EmptyRoom/>
    </v-main>
  </div>
</template>

<script>
import EmptyRoom from '@/components/ChatRoom/EmptyRoom.vue'
import NavBar from '../components/NavBar.vue'
export default {
    name: "HomeView",
    data: () => ({
        username: "airbussssss",
        usericon: "mdi-airplane",
        drawer: null,
        friends: [
            ["mdi-star", "gloria"],
            ["mdi-nut", "nut"],
        ],
        commu: [
            ["mdi-brain", "Dont think, Just DO"]
        ]
    }),
    components: { EmptyRoom, NavBar }
}
</script>
