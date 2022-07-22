<template>
  <div class="home">

    <v-navigation-drawer
      v-model="drawer"
      class = "brown darken-3 overfloe-y-auto"
      dark
      app
    >
      <h1 class = "white--text text-h2 font-weight-black" to="/">Hello
      <v-icon size ="40" color="amber">
        mdi-hand-wave
      </v-icon>
      </h1>
      <h2 class = "white--text text-left overflow-auto pl-3 pb-2">{{ username }}</h2>
      
      <v-divider></v-divider>

      <v-list-group
          :value="true"
          no-action
          class = "pt-2"
        >
          <template v-slot:activator>
            <v-list-item-content>
              <v-list-item-title class = "text-left white--text text-h4 font-weight-medium">Friends</v-list-item-title>
            </v-list-item-content>
          </template>

        <v-list subheader flat>
        <v-list-item
          v-for="[name] in friends"
          :key="name"
        >
          <v-list-item-content>
            <v-list-item-title class = "text-left text-h6">{{ name }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        </v-list>
      </v-list-group>

      <v-divider></v-divider>

      <div>
      <h3 class = "text-left white--text text-h4 font-weight-medium pl-3 pt-2">
        Community</h3>
      <v-list flat>
        <v-list-item
          v-for="[icon, name] in commu"
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
      </div>

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
            ["gloria"],
            ["nut"]
        ],
        commu: [
            ["mdi-brain", "Dont think, Just DO"]
        ]
    }),
}
</script>
