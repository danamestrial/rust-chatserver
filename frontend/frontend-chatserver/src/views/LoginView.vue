<template>
   <v-app id="login">
      <v-content>
         <v-container fluid fill-height>
            <v-layout align-center justify-center>
               <v-flex xs12 sm8 md4>
                  <v-card class="elevation-12">
                     <v-toolbar dark color="green">
                        <v-toolbar-title>LOGIN</v-toolbar-title>
                     </v-toolbar>
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
export default {
   name: "LoginPage",
   props: {
      source: String,
   },
};
</script>

<style></style>
