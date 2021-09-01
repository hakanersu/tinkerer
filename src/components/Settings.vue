<template>
  <div class="row">
    <label for="path">Project path</label>
    <input
      type="text"
      name="path"
      v-model="path"
    >
    <button @click="open">Select</button>
  </div>
</template>

<script>
import {dialog} from "@tauri-apps/api";
export default {
  data: () => ({
    path: ''
  }),
  mounted () {
    this. path = localStorage.getItem('tinker_path')
  },
  methods: {
    async open () {
     await dialog.open({ directory: true}).then(res => {
       this.path = res
        localStorage.setItem('tinker_path', res)
     }) 
    }
  }
}
</script>

<style>
.row {
  display: flex;
  flex-direction: column;
}
.row label {
  margin-bottom: 3px;
  display: block;
}
.row input {
  width: 50%;
  margin: 0;
  padding: 1.4em 1.4em 1.4em 1.4em;
  text-indent: 1em;
  border: 0;
  border-radius: 4px;
  background: #252830;
  color: white;
  margin-top: 3px;
}
</style>