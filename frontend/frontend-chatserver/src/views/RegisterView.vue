<template>
   <v-app id="register">
      <v-main>
         <v-container fluid fill-height>
            <v-layout align-center justify-center>
               <v-flex xs12 sm8 md4 >
                  <v-card class="elevation-12">
                  <v-col cols="12">
                     <v-img
                        src="../assets/circle.png"
                        class="pa-md-10 mx-lg-auto"
                        contain
                        width="260"
                        ></v-img>
                  </v-col>
                    <h1>SIGN UP</h1>
                     <v-card-text>
                        <v-form>
                            <v-col
                                cols="12"
                                sm="15"
                            >
                           <v-text-field
                              v-model="username"
                              label="Username"
                              maxlength="15"
                              counter="15"
                              filled
                              rounded
                              type="text"
                              required
                           ></v-text-field>
                           </v-col>
                            <v-col
                                cols="12"
                                sm="15"
                            >
                           <v-text-field
                              v-model="password"
                              label="Password"
                              :append-icon="show1 ? 'mdi-eye' : 'mdi-eye-off'"
                              :rules="[rules.required, rules.min]"
                              :type="show1 ? 'text' : 'password'"
                              filled
                              rounded
                              required
                              @click:append="show1 = !show1"
                           ></v-text-field>
                           </v-col>
                            <v-col
                                cols="12"
                                sm="15"
                            >
                           <v-text-field
                              v-model="confirmPassword" 
                              label="Confirm Password"
                              :append-icon="show2 ? 'mdi-eye' : 'mdi-eye-off'"
                              :rules="[rules.required, passwordconfirmrule]"
                              :type="show2 ? 'text' : 'password'"
                              filled
                              rounded
                              required
                              @click:append="show2 = !show2"
                           ></v-text-field>
                           </v-col>
                        </v-form>
                     </v-card-text>
                    <v-row
                        justify="space-around"
                    >
                    <v-btn
                    text
                    color="red"
                    to="login"
                    >Already have an account? Login here</v-btn>
                    </v-row>
                     <v-card-actions>
                        <v-spacer></v-spacer>
                            <v-col
                                cols="12"
                                sm="15"
                            >
                        <v-btn color="green"
                            class="ma-2 white--text"
                            rounded
                            x-large
                            @click="register()">Sign Up</v-btn>
                        </v-col>
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
   name: "RegisterPage",
   
   data: () => ({
    username: "",
    password: "",
    confirmPassword: "",
        show1: false,
        show2: false,
        rules: {
          required: value => !!value || 'Required.',
          min: v => v.length >= 8 || 'Min 8 characters',
        }
   }),

   methods: {
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
        this.$router.push("/login");
    },
    checksame(v) {
         return v == this.pasword;
      }
   },

   computed: {
      passwordconfirmrule() {
         return () => (this.password === this.confirmPassword) || 'Password Must match'
      }
   }
};
</script>