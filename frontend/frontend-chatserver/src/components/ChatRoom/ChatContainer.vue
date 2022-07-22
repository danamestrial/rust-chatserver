<template>
    <v-app class="yellow lighten-4">
    <div id="chat-container" class="chat-container">
        <div id="chat" class="overflow-y-auto">
            <ChatMsg/>
        </div>
        
        <div class="chat-input-container">
            <TextInput />
        </div>
    </div>
    </v-app>
</template>
<script>
import TextInput from '../MSG/TextInput'
import ChatMsg from '../MSG/ChatMsg.vue'
export default {
    name: "ChatContainer",
    components: {
    TextInput,
    ChatMsg
}
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
</style>