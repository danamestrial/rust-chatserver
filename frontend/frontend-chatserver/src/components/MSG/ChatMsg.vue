<template>
    <v-container>
    <v-row class ="mt-15 chat-list-container" >
        <v-list-item v-for="msg in chat" :key="msg">
        <v-list-item-content class="ml-1 py-0 ">
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
</template>

<script>
export default {
    data: () => ({
        chat:[
            {senderName:"gloria", role:"Support", text:"kik",timestamp:"12:23, 3 Jul 2022"},
            {senderName:"airbus", role:"Carry", text:"MK mai", timestamp:"12:30, 3 Jul 2022"},
            {senderName:"nut", role:"Support", text:"okk",timestamp:"12:35, 3 Jul 2022"}
        ],
        username:"airbus",
    })
}
</script>

<style scoped>
.chat-list-container {
    height: calc(100% - 120px); 
    overflow-y: auto;
}
</style>