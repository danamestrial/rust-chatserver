<template>
  <div class="home">

    <v-navigation-drawer
      v-model="drawer"
      class = "brown darken-3 overfloe-y-auto"
      dark
      app
      
    >

      <h1 class = "white--text text-h2 font-weight-black">Hello
      <v-icon size ="40" color="amber">
        mdi-hand-wave
      </v-icon>
      </h1>
      <h2 class = "white--text text-left overflow-auto">{{ username }}</h2>
      
      <v-avatar
        class="mb-4"
        size="90"
        color = "light-blue darken-3"
      >
      <v-icon size="45">{{ usericon }}</v-icon>
      </v-avatar>

      <v-divider></v-divider>

      <v-list-group
          :value="true"
          no-action
        >
          <template v-slot:activator>
            <v-list-item-content>
              <v-list-item-title class = "text-left white--text text-h5 font-weight-medium">Friends</v-list-item-title>
            </v-list-item-content>
          </template>

        <v-list subheader>
        <v-list-item
          v-for="[icon, name] in friends"
          :key="icon"
          link
          router to="/chat"
        >
          <v-list-item-icon>
            <v-icon>{{ icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title class = "text-left">{{ name }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        </v-list>
      </v-list-group>

      <v-divider></v-divider>

      <h1 class = "text-left white--text text-h4 font-weight-medium">Community</h1>
      <v-list>
        <v-list-item
          v-for="[icon, name] in commu"
          :key="icon"
          link
        >
          <v-list-item-icon>
            <v-icon>{{ icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title class = "text-left">{{ name }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>

      

      <v-btn 
      flat 
      absolute
      right
      bottom 
      color = "red"
      router to="/login">
        <span>Sign Out</span>
        <v-icon right>mdi-logout</v-icon>
      </v-btn>

    </v-navigation-drawer>
    
    <v-app-bar
    color = "light-blue lighten-3"
    flat
    app
    >
      <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
    
    </v-app-bar>

  </div>
</template>

<script>
export default {
    name: "HomeView",
    data: () => ({
        username: "airbussssss",
        usericon: "mdi-airplane",
        drawer: null,
        friends: [
            ["mdi-star", "gloria"],
            ["mdi-nut", "nut"],
        ],
        commu: [
            ["mdi-brain", "Dont think, Just DO"]
        ]
    }),
}
</script>
