<template>
    <v-app class="yellow lighten-4">
    <div id="chat-container" class="chat-container">
        <div class="chat-input-container">
            <TextInput />
        </div>
    </div>
    </v-app>
</template>
<script>
import TextInput from '../MSG/TextInput'
// import ChatList from '../MSG/ChatList'

export default {
    name: "ChatContainer",
    components: {
        TextInput
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