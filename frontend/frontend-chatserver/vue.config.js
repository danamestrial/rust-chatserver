// const { defineConfig } = require('@vue/cli-service')
// module.exports = defineConfig({
//   transpileDependencies: [
//     'vuetify'
//   ]
// })
module.exports = {
  devServer: {
    proxy: {
      "^/api": {
        target: "http://127.0.0.1:8000",
        ws: true,
        changeOrigin: true,
        headers: { 'Access-Control-Allow-Origin': '*' },
      },
    },
  },
};
