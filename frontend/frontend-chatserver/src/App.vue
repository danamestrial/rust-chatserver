<template>
  <v-app>
    <v-app-bar app color="primary" dark>
      <v-btn @click="sub()"> sub
      </v-btn>

      <v-btn @click="test()"> send
      </v-btn>

      <v-btn @click="pingServer()"> test
      </v-btn>

      <v-spacer></v-spacer>

      <v-btn href="https://github.com/vuetifyjs/vuetify/releases/latest" target="_blank" text>
        <span class="mr-2">Latest Release</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>
    </v-app-bar>

    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script>
import axios from "axios";

export default {
  name: "App",
  data: () => ({
    api_check: "not ping",
  }),

  methods: {
    sub() {
      var retryTime = 1;
      const events = new EventSource('/api/events');

      events.addEventListener("message", (ev) => {
        console.log("raw data", JSON.stringify(ev.data));
        console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
        // const msg = JSON.parse(ev.data);
      });

      events.addEventListener("open", () => {
        // setConnectedStatus(true);
        console.log(`connected to event stream at /api/events`);
        // retryTime = 1;
      });

      events.addEventListener("error", () => {
        events.close();

        let timeout = retryTime;
        retryTime = Math.min(64, retryTime * 2);
        console.log(`connection lost. attempting to reconnect in ${timeout}s`);
        // setTimeout(() => connect('/api/events'), (() => timeout * 1000)());
      });
    },

    async test() {
      const response = await axios
        .post("/api/message", {
          room: "1",
          userid: 1,
          message: "hello world"
        }).catch((error) => {
          if (error.response) {
            console.warn("something went wrong");
          }
        })
      console.log(response.data)
    }
  },

}
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