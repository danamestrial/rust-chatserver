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
                                    v-model="firstname"
                                    label="First name"
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
                                    v-model="lastname"
                                    label="Last Name"
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
                              filled
                              rounded
                              type="password"
                              required
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
                            to="login">Sign Up</v-btn>
                        </v-col>
                     </v-card-actions>
                  </v-card>
               </v-flex>
            </v-layout>
         </v-container>
      </v-main>
    </v-app>
</template>