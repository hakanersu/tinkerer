<template>
  <div class="input left-pane"><textarea ref="codeEditor" /></div>
</template>

<script>
import 'codemirror/mode/php/php';
import '../assets/material-palenight.css';
import CodeMirror from 'codemirror';

export default {
  data: () => ({
    value: '',
    codeEditor: null,
  }),

  props: ['path'],

  mounted () {
    const config = {
      autofocus: true,
      extraKeys: {
        'Cmd-Enter': () => {
          this.executeCode();
        },
        'Ctrl-Enter': () => {
          this.executeCode();
        },
      },
      indentWithTabs: true,
      lineNumbers: true,
      lineWrapping: true,
      mode: 'text/x-php',
      tabSize: 4,
      theme: 'material-palenight',
    };

    this.codeEditor = CodeMirror.fromTextArea(this.$refs.codeEditor, config);
    this.codeEditor.on('change', editor => {
      localStorage.setItem('tinker-tool', editor.getValue());
    });
    let value = localStorage.getItem('tinker-tool');
    if (typeof value === 'string') {
      this.codeEditor.setValue(value);
      this.codeEditor.execCommand('goDocEnd');
    }
  },
  methods: {
    executeCode () {
      let code = this.codeEditor.getValue().trim();
      if (code === '') {
        this.$emit('execute', '<error>You must type some code to execute.</error>');
        return;
      }
      this.$emit('execute', code)
    },
  },
};
</script>

<style src="codemirror/lib/codemirror.css" /> 
