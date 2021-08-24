<template>
  <div class="input left-pane"><textarea ref="codeEditor"></textarea></div>
</template>

<script>
import 'codemirror/mode/php/php';
import '../assets/material-palenight.css';
import CodeMirror from 'codemirror';

export default {
  props: ['value'],
  data: () => ({
    codeEditor: null,
    result: '',
  }),
  watch: {
      value (newValue) {
           this.codeEditor.setValue(newValue);
    this.codeEditor.execCommand('goDocEnd');
      }
  },
  mounted () {
    const config = {
      indentWithTabs: true,
      lineNumbers: true,
      lineWrapping: true,
      mode: 'text/x-php',
      tabSize: 4,
      theme: 'material-palenight',
      readOnly: 'nocursor'
    };
    this.codeEditor = CodeMirror.fromTextArea(this.$refs.codeEditor, config);
    this.codeEditor.setValue(this.value);
    this.codeEditor.execCommand('goDocEnd');
  }
};
</script>

<style src="codemirror/lib/codemirror.css" /> 
