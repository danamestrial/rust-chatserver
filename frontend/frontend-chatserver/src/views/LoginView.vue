<template>
   <v-app id="login">
      <v-content>
         <v-container fluid fill-height>
            <v-layout align-center justify-center>
               <v-flex xs12 sm8 md4 >
                  <v-card class="elevation-12">
                  <v-col cols="">
                     <v-img
                        :src="require('../assets/circle.png')"
                        class="pa-md-10 mx-lg-auto"
                        contain
                        width="260"
                        />
                  </v-col>
                    <h1>LOGIN</h1>
                     <v-card-text>
                        <v-form>
                            <v-col
                                cols="12"
                                sm="15"
                                md="40"
                            >

                           <v-text-field
                              name="Username"
                              label="Username"
                              maxlength="15"
                              counter="15"
                              filled
                              rounded
                              type="text"
                           ></v-text-field>
                           </v-col>
                            <v-col
                                cols="12"
                                sm="15"
                                md="40"
                            >
                           <v-text-field
                              id="password"
                              name="password"
                              label="Password"
                              filled
                              rounded
                              type="password"
                           ></v-text-field>
                           </v-col>
                        </v-form>
                     </v-card-text>
                    <v-row
                        align="center"
                        justify="space-around"
                    >
                    <v-btn
                    text
                    color="red"
                    >Register here</v-btn>
                    <v-btn
                    text
                    color="red"
                    >Forgot your password?</v-btn>
                    </v-row>
                     <v-card-actions>
                        <v-spacer></v-spacer>
                            <v-col
                                cols="12"
                                sm="15"
                                md="40"
                            >
                        <v-btn color="green"
                            class="ma-2 white--text"
                            rounded
                            x-large
                            to="/">Let's go!
                            </v-btn>
                            </v-col>
                     </v-card-actions>
                  </v-card>
               </v-flex>
            </v-layout>
         </v-container>
      </v-content>
   </v-app>
</template>

<script>
import axios from "axios";

export default {
   name: "LoginPage",
   
   data: () => ({
    username: "",
    password: "",
   }),

   methods: {
    async login() {

        const response = await axios
        .post("/api/login", {
            username: this.username,
            password: this.password
        })
        .catch((error) => {
            if (error.response) {
                console.warn("something went wrong");
            }
        });
        console.log(response.data);

        if (response.data === true) {
            window.location = "/";
        }
    },

    async register() {

        const response = await axios
        .post("/api/register", {
            username: this.username,
            password: this.password
        })
        .catch((error) => {
            if (error.response) {
                console.warn("something went wrong");
            }
        });
        console.log(response.data);
    }
   }
};
</script>