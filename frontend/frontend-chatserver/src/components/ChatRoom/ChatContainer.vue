<template>
    <v-app class="yellow lighten-4">
    <div id="chat-container" class="chat-container">
        <div id="chat" class="overflow-y-auto">
            <v-container>
                <v-row class ="mt-15 chat-list-container" >
                    <v-list-item v-for="msg in chat" :key="msg">
                    <v-list-item-content class="ml-1 py-2 ">
                        <v-list-item-title class="text-left">{{msg.senderName}} - {{msg.role}}</v-list-item-title>
                        <v-card
                            class="d-flex pa-2 yellow darken-1" 
                            rounded-xl
                            flat
                            v-if="msg.senderName!=username"
                            >
                            <div>
                            {{msg.text}}</div>
                        </v-card>

                        <v-card
                        class="d-flex pa-2 light-green lighten-1" 
                        rounded-xl
                        flat
                        v-if="msg.senderName==username"
                        >
                        <div>{{msg.text}}</div>
                        </v-card>

                        <v-list-item-subtitle class="text-right caption pa-0">{{msg.timestamp}}</v-list-item-subtitle>
                    </v-list-item-content>
                    </v-list-item>
                
                    
                </v-row>
            </v-container>
        </div>
        
        <div class="chat-input-container">
            <v-container class="d-flex align-center chat-input">
                <v-textarea
                v-model="text"
                class="message"
                solo
                no-resize
                rows="2"
                name="message"
                label="Message"
                ></v-textarea>
                <v-btn icon class="ml-3" @click="sender">
                <v-icon>mdi-send</v-icon></v-btn>
            </v-container>
        </div>
    </div>
    </v-app>
</template>

<script>
import moment from 'moment';
import axios from 'axios';
import {getRoom} from '../../views/ChatView.vue'

export default {
    data: () => ({
        chat:[
            {senderName:"gloria", role:"Support", text:"kik",timestamp:"12:23, 3 Jul 2022"},
            {senderName:"airbus", role:"Carry", text:"MK mai", timestamp:"12:30, 3 Jul 2022"},
            {senderName:"nut", role:"Support", text:"okk",timestamp:"12:35, 3 Jul 2022"}
        ],
        username:"airbus",
        role:"Carry",
        text: null
    }),
    methods: {
        sender() {
            axios.post("api/message", {
                room: getRoom(),
                userid: "1",
                message: this.text,
            })
        },
        send: function(){
            this.chat.push(
        {
        senderName: this.username,
        role:this.role,
        text: this.text,
        timestamp:moment().format("h:mm a, Do MMM YYYY")
        })
        this.text = null
        }
    },
    mounted() {
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
      .then(() => {console.log("Connected to events handle")})
      .catch((err) => console.error("Failed make initial connection:", err));
  },
}
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