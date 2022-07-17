<template>
    <v-app id="inspire">
      <v-main>
         <v-container fluid fill-height>
            <v-layout align-center justify-center>
               <v-flex xs12 sm8 md4>
                  <v-card class="elevation-12">
                     <v-toolbar dark color="primary">
                        <v-toolbar-title>Login form</v-toolbar-title>
                     </v-toolbar>
                     <v-card-text>
                        <v-form>
                           <v-text-field
                              v-model="username"
                              label="Login"
                              type="text"
                           ></v-text-field>
                           <v-text-field
                              v-model="password"
                              label="Password"
                              type="password"
                           ></v-text-field>
                        </v-form>
                     </v-card-text>
                     <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn color="primary" @click="register()">Login</v-btn>
                     </v-card-actions>
                  </v-card>
               </v-flex>
            </v-layout>
         </v-container>
      </v-main>
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