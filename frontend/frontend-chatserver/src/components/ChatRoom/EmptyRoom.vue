<template>
    <v-main fill-height fluid>
        <v-row align-content="center"
            justify="center">
            <v-col align-content="center">
                <h3 class="display-1 font-weight-light">Please select a chat room</h3>
                <v-dialog
                v-model ="dialog"
                persistent
                max-width="400px"
                >
                    <template v-slot:activator="{on}">
                      <v-btn
                        class="mt-3"
                        abdolute
                        color ="blue"
                        rounded
                        v-on="on"
                        >
                        <v-icon class="white--text">mdi-plus</v-icon>
                        <span class="white--text">Join commu</span>
                        </v-btn>  
                    </template>
                    <v-card>
                    <v-card-title class="text-xs-center">Enter the commu ID</v-card-title>
                    <v-card-text>
                        <v-container>
                            <v-text-field label="commu ID" required></v-text-field>
                        </v-container>
                    </v-card-text>
                    <v-card-action>
                        <v-btn
                        color ="red"
                        text
                        @click="dialog = false"
                        >
                            Close
                        </v-btn>
                        <v-btn
                        color ="green"
                        text
                        @click="join()"
                        >
                            Join
                        </v-btn>
                    </v-card-action>
                    </v-card>
                </v-dialog>
            </v-col>
            
        </v-row>
    </v-main>
</template>

<script>
// import axios from "axios"
export default {
    name: "EmptyRoom",
    data: () => ({
        commuID: "",
        dialog: false,
    }),

    // method:{
    //     async join() {
    //         const response = await axios
    //         .post("api/commu", {
    //             commuID:this.commuID
    //         })
    //         .catch((error) => {
    //         if (error.response) {
    //             console.warn("cannot find this community");
    //         }
    //         });
    //         if(response.data == true){

    //         }
    //     }
    // }
}
</script>