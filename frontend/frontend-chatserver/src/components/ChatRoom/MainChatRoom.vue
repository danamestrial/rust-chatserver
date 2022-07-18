<template>
    <v-container fluid class="fill-height message-container" v-if="!isRequiresLogin">
        <v-row
            justify="center"
            align="center"
            v-if="!isRoomSelected"
        >
            <v-col class="col-xs-12"><EmptyRoom /></v-col>
        </v-row>
        <ChatContainer v-if="isRoomSelected" />
        
    </v-container>
</template>
<script>
import { mapState, mapActions } from 'vuex';
import EmptyRoom from './EmptyRoom.vue';
import ChatContainer from './ChatContainer.vue';
export default {
    name: "MessageContainer",
    components: {
        ChatContainer, EmptyRoom
    },
    computed: {
        ...mapState('userModule', {
            userState: state => state.user
        }),
        ...mapState('roomModule', {
            roomID: state => state.activeRoom
        }),
        isRoomSelected: function() {
            return this.roomID;
        }
    },
    methods: {
        ...mapActions('roomModule', ['clearRoom'])
    },
    created() {
        // Clear room selection
        this.clearRoom();
    }
}
</script>
<style scoped>
.message-container {
    padding: 0;
}
</style>