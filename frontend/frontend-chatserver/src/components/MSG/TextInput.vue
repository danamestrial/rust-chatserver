<template>
    <v-container class="chat-input">
        <v-textarea
          v-model.trim="message"
          v-on:keyup.enter.exact="submitHandler"
          class="message"
          solo
          no-resize
          rows="2"
          name="message"
          label="Message"
        ></v-textarea>
    </v-container>
</template>
<script>
import { mapActions } from 'vuex';

export default {
    name: "ChatInput",
    data () {
        return {
            message: ''
        }
    },
    methods: {
        ...mapActions('roomModule', ['sendMessage']),
        async submitHandler( ) {
            if(this.message.length > 0) {
                const {message} = this;
                this.sendMessage({message}); 
            }   
            this.resetMessage();        
        },
        resetMessage() {
            this.message = '';
        }
    }
}
</script>