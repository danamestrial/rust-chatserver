<template>
  <v-main fill-height fluid>
    <v-row align-content="center" justify="center">
      <v-col align-content="center">
        <h3 class="display-1 font-weight-light">
          Please select a chat room or join a new one
        </h3>

        <v-spacer></v-spacer>

        <v-dialog id="join" v-model="dialog2" persistent max-width="400px">
          <template v-slot:activator="{ on }">
            <v-btn class="mt-3" abdolute color="green" rounded v-on="on">
              <v-icon class="white--text">mdi-plus</v-icon>
              <span class="white--text">Join commu</span>
            </v-btn>
          </template>
          <v-card>
            <v-card-title class="text-xs-center">Enter room name</v-card-title>
            <v-card-text>
              <v-container>
                <v-text-field v-model="roomname" required></v-text-field>
              </v-container>
            </v-card-text>
            <v-card-action>
              <v-btn color="red" text @click="dialog2 = false"> Close </v-btn>
              <v-btn color="green" text @click="addRoom()"> Join </v-btn>
            </v-card-action>
          </v-card>
        </v-dialog>

        <v-spacer></v-spacer>
      </v-col>
    </v-row>
  </v-main>
</template>

<script>
import axios from "axios";
export default {
  name: "EmptyRoom",
  data: () => ({
    roomname: "",
    dialog: false,
    dialog2: false,
    dialog3: false,
    icons: [
      "mdi-brain",
      "mdi-cat",
      "mdi-dog",
      "mdi-cheese",
      "mdi-cookie",
      "mdi-cards-playing",
      "mdi-food-turkey",
      "mdi-music",
      "mdi-bee",
    ],
  }),

  methods: {
    async addRoom() {
      this.dialog2 = false;
      if (!this.$store.state.storage.has(this.roomname)) {
        await axios.post("/api/addroom", {
          username: this.$store.state.username,
          room: this.roomname,
        });
      }
      this.$router.go();
    },
  },

  mounted() {
    if (!this.$store.state.status) {
      window.location = "/login";
    }
  },
};
</script>