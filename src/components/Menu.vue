<template>
  <div class="main-menu">
    <hsc-menu-style-black ref="menu">
      <hsc-menu-bar style="border-radius: 0 0 4pt 0;">
        <hsc-menu-bar-item label="File">
          <hsc-menu-item
            label="Tinker"
            @click="$emit('page', 'default')"
            :sync="true"
          />
          <hsc-menu-separator />
          <hsc-menu-item
            label="Settings"
            :sync="true"
            @click="$emit('page', 'settings')"
          />
          <hsc-menu-item
            label="Exit"
            @click="$emit('action', 'exit')"
          />

        </hsc-menu-bar-item>
        <hsc-menu-bar-item label="Edit">
          <hsc-menu-item
            label="Undo"
            keybind="meta+z"
            @click="window.alert('Undo')"
          />
          <hsc-menu-separator />
          <hsc-menu-item
            label="Cut"
            keybind="meta+x"
            @click="window.alert('Cut')"
          />
          <hsc-menu-item
            label="Copy"
            keybind="meta+c"
            @click="window.alert('Copy')"
          />
          <hsc-menu-item
            label="Paste"
            keybind="meta+v"
            @click="window.alert('Paste')"
            :disabled="true"
          />
        </hsc-menu-bar-item>
      </hsc-menu-bar>
    </hsc-menu-style-black>

  </div>
</template>

<script>
export default {
  mounted () {
    console.log(this.$refs.menu.$data)
  }
}
</script>
<style>
.main-menu .fixed {
  z-index: 9999;
}
.main-menu {
  background-color: rgb(32, 32, 32);
}
.main-menu .menubar {
  box-shadow: none !important;
  border-radius: 0 !important;
}
</style>