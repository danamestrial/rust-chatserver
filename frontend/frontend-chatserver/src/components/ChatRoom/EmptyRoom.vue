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
                
                <v-spacer></v-spacer>

                <v-dialog
                v-model ="dialog2"
                persistent
                max-width="400px"
                >
                    <template v-slot:activator="{on}">
                      <v-btn
                        class="mt-3"
                        abdolute
                        color ="green"
                        rounded
                        outlined
                        v-on="on"
                        >
                        <v-icon class="green--text">mdi-plus</v-icon>
                        <span class="green--text">Create Commu</span>
                        </v-btn>  
                    </template>
                    <v-card>
                    <v-card-text>
                        <v-container>
                            <p class="black--text text-h5 text-left">Creating new community</p>
                            <v-text-field label="Enter new Community name" required></v-text-field>
                            <v-select
                                outlined
                                v-model="members"
                                :items="icons"
                                attached
                                label="Select icon"
                            >
                            </v-select>
                            <v-select
                                outlined
                                v-model="members"
                                :items="friends"
                                attached
                                chips
                                multiple
                                label="Select friends"
                            >
                            </v-select>
                        </v-container>
                    </v-card-text>
                    <v-card-action>
                        <v-btn
                        color ="red"
                        text
                        @click="dialog2 = false"
                        >
                            Close
                        </v-btn>
                        <v-btn
                        color ="green"
                        text
                        @click="create()"
                        >
                            Create
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
        dialog2: false,
        icons:["mdi-brain","mdi-cat",
                "mdi-dog","mdi-cheese",
                "mdi-cookie","mdi-cards-playing",
                "mdi-food-turkey","mdi-music",
                "mdi-bee",],
        friends: ["gloria","nut"]
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