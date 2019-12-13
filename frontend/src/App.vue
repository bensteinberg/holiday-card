<template>
  <div id="app">
    <transition name="fade">
      <div :key="picture.file_name" v-if="picture">
        <img :src="`/pictures/${picture.file_name}`" v-if="picture">
        <p id="caption">
          <span v-if="picture.caption" style="font-size: 1.5em">{{picture.caption}}<br></span>
          <span v-if="picture.source"  style="font-style: italic">— {{picture.source}}</span>
        </p>
      </div>
    </transition>
  </div>
</template>

<script>
export default {
  name: 'app',
  data() {
    return {
      picture: null
    }
  },
  async mounted() {
    this.getPicture();
    // read speed=N cookie; default to 5
    const speed = parseInt(document.cookie.replace(/(?:(?:^|.*;\s*)speed\s*=\s*([^;]*).*$)|^.*$/, "$1")) || 5;
    window.setInterval(this.getPicture, speed * 1000)
  },
  methods: {
    async getPicture() {
      const response = await fetch("/api/foo");
      const data = await response.json();
      if (this.picture && data.file_name === this.picture.file_name)
        this.getPicture();
      else
        this.picture = data;
    }
  }
}
</script>

<style>
  /* add blank frame around picture */
  *{box-sizing: border-box;}

  /* make image fit to available space */
  html, body, #app{
    width: 100%;
    height: 100%;
    margin: 0;
  }
  body {
  }
  #app{
    position: relative;
    /* copied from the default vue app template */
    font-family: 'Avenir', Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
  }
  #app > div {
    position: absolute;
    width: 100%;
    height: 100%;
    padding: 5%;  /* change to 0 if no frame desired */
  }
  img {
    object-fit: contain;  /* change to "cover" to zoom until image covers whole frame */
    object-position: 50% 50%;  /* if using "cover", use this to slide left-right or up-down */
    width: 100%;
    height: 100%;
  }
  /*.fade-enter-active, .fade-leave-active {*/
  /*  transition: opacity .5s;*/
  /*}*/
  /*.fade-enter, .fade-leave-to !* .fade-leave-active below version 2.1.8 *! {*/
  /*  opacity: 0;*/
  /*}*/
  .fade-enter-active, .fade-leave-active {
    transition: opacity .5s ease-in-out;
  }
  .fade-enter-to{
    opacity: 1;
  }
  .fade-enter{
    opacity: 0;
  }
  #caption {
    position: absolute;
    bottom: 0;
    width: 100%;
    background: white;
    padding: .5em;
    margin: 0;
    left: 0;
  }
</style>
