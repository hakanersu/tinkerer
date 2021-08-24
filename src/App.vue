<template>
 <tinker />
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'
import Tinker from './components/Tinker.vue'
export default {
  name: 'App',
  components: {
    Tinker
  },
  data: () => ({
    result: '',
    code: '',
  }),
  mounted () {
    this.run("[1,2,3,4]")
  },
  methods: {
    execute () {
      this.run(this.code)
    },
    run (command) {
      invoke('tinker', { command: command }).then(resp => {
        console.log(resp)
        this.result = resp
      })
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
