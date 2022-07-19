<template>
<div class="chatView">
    <NavBar/>
    <v-content>
        <MainChatRoom />
    </v-content>
</div>
</template>

<script>
import NavBar from '@/components/NavBar.vue'
import MainChatRoom from'../components/ChatRoom/ChatContainer.vue'
export default {
    name: "ChatView",
    data: () => ({
    }),

    components: { MainChatRoom,NavBar }
}
</script>