<template>
  <div class="row">
    <label for="path">Project path</label>
    <input
      type="text"
      name="path"
    >
  </div>
</template>

<script>
export default {

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