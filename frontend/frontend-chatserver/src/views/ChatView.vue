<template>
<v-app>
    <div class="chatView">
        <NavBar/>
        <div id="chat-container">
            <v-main class="pt-0">
                <ChatContainer />
            </v-main>
        </div>
    </div>
</v-app>
</template>

<script>
import NavBar from '@/components/NavBar.vue'
import ChatContainer from'@/components/ChatRoom/ChatContainer.vue'
export default {
    name: "ChatView",
    data: () => ({
        commuName:"Don't think, JUST DO"
    }),
    methods: {
        getRoom() {
            return this.commuName;
        }
    },
    components: { ChatContainer, NavBar },
};

export function getRoom() {
    return this.commuName
}
</script>