<template>
  <v-app class="yellow lighten-4">
  
    <div id="chat-container" class="chat-container">
      <div id="chat">
        <v-container
          id="scroll-target"
          style="max-height: 600px"
          class="overflow-y-auto">

          <v-row
            v-chat-scroll="{ enable: false }"
            class="mt-15 chat-list-container">
            <v-list-item v-for="msg in chat" :key="msg.senderName">
              <v-list-item-content class="ml-1 py-2">
                <v-list-item-title class="text-left"
                  >{{ msg.senderName }} - {{ msg.role }}</v-list-item-title
                >
                <v-card
                  class="d-flex pa-2 yellow darken-1"
                  rounded-xl
                  flat
                  v-if="msg.senderName != username"
                >
                  <div>
                    {{ msg.text }}
                  </div>
                </v-card>

                <v-card
                  class="d-flex pa-2 light-green lighten-1"
                  rounded-xl
                  flat
                  v-if="msg.senderName == username"
                >
                  <div>{{ msg.text }}</div>
                </v-card>

                <v-list-item-subtitle class="text-right caption pa-0">{{
                  msg.timestamp
                }}</v-list-item-subtitle>
              </v-list-item-content>
            </v-list-item>
          </v-row>
        </v-container>
      </div>

      <div class="chat-input-container">
        <v-container class="d-flex align-center chat-input">
          <v-textarea
            v-model="text2"
            class="message"
            solo
            no-resize
            rows="2"
            name="message"
            label="Message"
          ></v-textarea>
          <v-btn icon class="ml-3" @click="sender">
            <v-icon>mdi-send</v-icon></v-btn
          >
        </v-container>
      </div>
    </div>
  </v-app>
</template>

<script>
import moment from "moment";
import axios from "axios";
import store from "@/store";

let sseclient;

export default {
  data: () => ({
    chat: store.state.storage.get(store.state.roomname),
    username: store.state.username,
    role: "Carry",
    text2: null,
  }),
  watch: {
    "$store.state.roomname": function () {
      this.chat = store.state.storage.get(store.state.roomname);
    },
  },
  methods: {
    async sender() {
      await axios.post("api/message", {
        room: store.state.roomname,
        username: store.state.username,
        message: this.text2,
      });
    },
    send(username, message, room) {
      //   console.log("yes", room);
      if (message != "") {
        const msg = {
          senderName: username,
          role: this.role,
          text: message,
          timestamp: moment().format("h:mm a, Do MMM YYYY"),
        };
        this.$store.commit("addMessage", { message: msg, room: room });
        this.text2 = "";
      }
    },
  },
  created() {
    sseclient = this.$sse.create({
      url: "http://localhost:8000/api/events",
      format: "json",
      //withCredentials: true,
      // polyfill: true,
    });
    sseclient.on("message", (msg) => {
      console.log(msg);
      //   console.log(msg.room);
      this.send(msg.username, msg.message, msg.room);
    });
    sseclient.on("error", (err) =>
      console.error("Failed to parse or lost connection:", err)
    );
    sseclient
      .connect()
      .then(() => {
        console.log("Connected to events handle");
      })
      .catch((err) => console.error("Failed make initial connection:", err));
  },
  beforeDestroy() {
    sseclient.disconnect();
  },
  mounted() {
    if(!store.state.status) {
      window.location = "/login";
    }
  },
};
</script>

<style scoped>
.chat-container {
  width: 100%;
}
.chat-input-container {
  background-color: #5ec4d2;
  width: inherit;
  height: 120px;
  position: absolute;
  bottom: 0;
}
.chat-list-container {
  height: calc(100% - 120px);
  overflow-y: auto;
}
</style>