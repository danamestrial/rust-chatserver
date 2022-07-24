<template>
  <div class="home">
    <NavBar />
    <v-main>
      <EmptyRoom />
    </v-main>
  </div>
</template>

<script>
import EmptyRoom from "@/components/ChatRoom/EmptyRoom.vue";
import NavBar from "../components/NavBar.vue";
import store from "@/store";

export default {
  name: "HomeView",
  title: "Circle OpenChat",
  data: () => ({
    username: "airbussssss",
    usericon: "mdi-airplane",
    drawer: null,
    friends: [
      ["mdi-star", "gloria"],
      ["mdi-nut", "nut"],
    ],
    commu: store.state.rooms,
  }),
  mounted() {
    if(!store.state.status) {
      window.location = "/login";
    }
  },
  components: { EmptyRoom, NavBar },
};
</script>
