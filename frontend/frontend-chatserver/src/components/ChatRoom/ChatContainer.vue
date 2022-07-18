<template>
    <div id="chat-container" class="chat-container">
        <div id="chat-list-container" class="chat-list-container">
            <ChatList />
        </div>
        <div class="chat-input-container">
            <TextInput />
        </div>
    </div>
</template>
<script>
import TextInput from '../MSG/TextInput';
import ChatList from '../MSG/ChatList';

export default {
    name: "ChatContainer",
    components: {
        ChatInput, ChatList
    }
}
</script>
<style scoped>
.chat-container {
    width: 100%;
}
.chat-list-container {
    height: calc(100% - 120px);  
    width: inherit;
    overflow-y: auto;
}
.chat-input-container {
    background-color: #eeeeee;
    width: inherit;
    height: 120px;
    position: absolute;
    bottom: 0;
}
</style>