<template>
  <div class="container">
    <tinker-menu @page="setPage" />
    <splitpanes
      class="default-theme"
      v-if="page === 'default'"
    >
      <pane>
        <code-input
          id="code-input"
          v-model="input"
          :path="path"
          @execute="handleExecute"
          class="flex-1"
          ref="code_input"
        />
      </pane>
      <pane>
        <code-output
          id="code-output"
          :value="output"
          class="flex-1"
          ref="code_output"
        />
      </pane>
    </splitpanes>
    <div
      v-if="page ==='settings'"
      class="settings"
    >
      <settings />
    </div>
  </div>
</template>

<script>
import CodeInput from './CodeInput';
import CodeOutput from './CodeOutput';
import TinkerMenu from './Menu.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { Splitpanes, Pane } from 'splitpanes'
import 'splitpanes/dist/splitpanes.css'
import Settings from './Settings.vue'
export default {
  components: {
    CodeInput,
    CodeOutput,
    Splitpanes,
    Pane,
    TinkerMenu,
    Settings
  },
  props: ['path'],
  data: () => ({
    input: '',
    page: 'default',
    output: '// Use cmd+enter or ctrl+enter to run.',
  }),
  mounted () {
    let path = localStorage.getItem('tinker_path')
    if (!path) {
      this.page  = 'settings'
    }
  },
  methods: {
    setPage (page) {
      this.page = page
    },
    handleExecute (output) {
      const path = localStorage.getItem('tinker_path')
      const bin_path = localStorage.getItem('bin_path') || '/usr/bin/php'
      invoke('tinker', { command: output , path: path, bin: bin_path}).then(resp => {
        this.output = resp
      })
    },
  },
};
</script>

<style>
body,
.container {
  margin: 0;
  padding: 0;
  width: 100vw;
  height: 100vh;
}
.container {
  color: #a6accd;
}
.settings {
  background-color: rgb(41, 45, 62);
  height: 100vh;
  padding: 15px;
}
.splitpanes {
  height: calc(100% - 24px) !important;
}
.splitpanes__pane {
  display: flex;
  justify-content: center;
  align-items: center;
  font-family: Helvetica, Arial, sans-serif;
}
.splitpanes.default-theme .splitpanes__splitter {
  background-color: #14161f;
}

.default-theme.splitpanes--vertical > .splitpanes__splitter,
.default-theme .splitpanes--vertical > .splitpanes__splitter {
  border-left: 1px solid #111;
}

.CodeMirror {
  height: 100%;
}
.left-pane,
.right-pane {
  width: 100%;
  height: 100%;
}
</style>