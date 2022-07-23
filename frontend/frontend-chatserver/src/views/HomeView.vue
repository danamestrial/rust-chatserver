<template>
  <div class="home">
    <NavBar />
    <v-app-bar id="appbar" color="light-blue lighten-3" flat app>
      <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
    </v-app-bar>
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
  data: () => ({
    username: "airbussssss",
    usericon: "mdi-airplane",
    drawer: null,
    friends: [
      ["mdi-star", "gloria"],
      ["mdi-nut", "nut"],
    ],
    commu: [["mdi-brain", "Dont think, Just DO"]],
  }),
  mounted() {
    if(!store.state.status) {
      window.location = "/login";
    }
    this.$sse
      .create({
        url: "http://localhost:8000/api/events",
        format: "json",
        //withCredentials: true,
        // polyfill: true,
      })
      .on("message", (msg) => console.info("Message:", msg))
      .on("error", (err) =>
        console.error("Failed to parse or lost connection:", err)
      )
      .connect()
      .then(() => {
        console.log("Connected to events handle");
      })
      .catch((err) => console.error("Failed make initial connection:", err));
  },
  components: { EmptyRoom, NavBar },
};
</script>
