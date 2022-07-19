<template>
<div class="chatView">
    <NavBar/>
    <div id="chat-container">
        <v-main>
            <ChatContainer />
        </v-main>
    </div>
</div>
</template>

<script>
import NavBar from '@/components/NavBar.vue'
import ChatContainer from'../components/ChatRoom/ChatContainer.vue'
export default {
    name: "ChatView",
    data: () => ({
    }),

    components: { ChatContainer,NavBar }
}
</script>