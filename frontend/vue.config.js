module.exports = {
  devServer: {
    // host: "127.0.0.1",
    port: 8000,
    proxy: {
      '^/': {
        target: 'http://127.0.0.1:8001',
        ws: true,
      }
    }
  }
};