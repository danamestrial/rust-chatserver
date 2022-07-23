<template>
<v-app>
    <v-app-bar
      app
      color="primary"
      dark
    >
     <v-btn @click="sub()"> sub </v-btn>

      <v-btn @click="test()"> send </v-btn>

      <v-btn @click="pingServer()"> test </v-btn>
      <div class="d-flex align-center">
        <v-img
          alt="Vuetify Logo"
          class="shrink mr-2"
          contain
          src="https://cdn.vuetifyjs.com/images/logos/vuetify-logo-dark.png"
          transition="scale-transition"
          width="40"
        />

        <v-img
          alt="Vuetify Name"
          class="shrink mt-1 hidden-sm-and-down"
          contain
          min-width="100"
          src="https://cdn.vuetifyjs.com/images/logos/vuetify-name-dark.png"
          width="100"
        />
      </div>

      <v-spacer></v-spacer>

      <v-btn
        href="https://github.com/vuetifyjs/vuetify/releases/latest"
        target="_blank"
        text
      >
        <span class="mr-2">Latest Release</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>
    </v-app-bar>
    
    <v-main class = "pa-0">
      <router-view/>
    </v-main>
  </v-app>
</template>

<script>
import axios from "axios";

export default {
  name: "App",
  // mounted() {
  //   this.$sse
  //     .create({
  //       url: "http://localhost:8000/api/events",
  //       format: "json",
  //       //withCredentials: true,
  //       // polyfill: true,
  //     })
  //     .on("message", (msg) => console.info("Message:", msg))
  //     .on("error", (err) =>
  //       console.error("Failed to parse or lost connection:", err)
  //     )
  //     .connect()
  //     .then(() => {console.log("Connected to events handle")})
  //     .catch((err) => console.error("Failed make initial connection:", err));
  // },

  methods: {
    test() {
    axios.post("api/message", {
      room: "1",
      userid: 69,
      message: "hello world",
    });
    },
    // test() {
    //   let formdata = new FormData();
    //   formdata.append("room", "1");
    //   formdata.append("username", "airbus");
    //   formdata.append("message", "hellowordl");
    //   axios.post("/api/message", formdata);
    // },
  },
};
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>