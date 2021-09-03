<template>
  <div>
    <div class="flex">
      <div class="row">
        <label for="path">Project path</label>
        <input
          type="text"
          name="project_path"
          v-model="project_path"
        >
      </div>
      <button
        @click="open"
        class="button"
      >Choose path</button>
    </div>
    <div
      class="flex"
      style="margin-top: 15px;"
    >
      <div class="row">
        <label for="path">PHP executable path</label>
        <input
          type="text"
          name="bin_path"
          v-model="bin_path"
        >
      </div>
      <button
        @click="saveBinPath"
        class="button"
      >Save</button>
    </div>
  </div>
</template>

<script>
import { dialog } from "@tauri-apps/api";
export default {
  data: () => ({
    project_path: '',
    bin_path: '/usr/bin/php',
  }),
  mounted () {
    this.project_path = localStorage.getItem('tinker_path')
  },
  methods: {
    async open () {
      await dialog.open({ directory: true }).then(res => {
        if (!res) {
          return false
        }
        this.project_path = res
        localStorage.setItem('tinker_path', res)
      })
    },
    saveBinPath () {
      localStorage.setItem('bin_path', this.bin_path)
    }
  }
}
</script>

<style>
.flex {
  display: flex;
  align-items: flex-end;
}
.row {
  display: flex;
  flex-direction: column;
  width: 600px;
}
.row label {
  margin-bottom: 3px;
  display: block;
}
.row input {
  margin: 0;
  padding: 5px;
  height: 40px;
  text-indent: 1em;
  border: 0;
  border-radius: 4px;
  background: #252830;
  color: white;
  margin-top: 3px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.5);
  box-sizing: border-box;
}
.button {
  box-sizing: border-box;
  border-radius: 4px;
  margin-left: 15px;
  height: 40px;
  border: 0;
  padding: 10px;
  background-color: #66abff;
  border-color: #66abff;
  color: #fff;
  font-weight: 600;
  cursor: pointer;
}
</style>