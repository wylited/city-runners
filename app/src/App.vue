<script setup lang="ts">
import Login from "./components/Login.vue";
import Game from "./components/Game.vue";
import { store } from "./store.ts"
import { onMounted } from 'vue'

onMounted(() => {
  document.documentElement.classList.add('dark')
})
</script>

<template>
  <div vaul-drawer-wrapper class="overscroll-none min-h-screen bg-background text-foreground">
    <div class="p-2">
      <component :is="store.page" />
    </div>
  </div>
</template>

