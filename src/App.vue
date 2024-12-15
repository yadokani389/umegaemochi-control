<script setup lang="ts">
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/core';

const ports = ref<string[]>([]);
</script>

<template>
  <main :class="$style.container">
    <h1>Welcome to umegaemochi-control</h1>

    <button @click="invoke('connect_server')">Click me</button>
    <ul v-if="ports.length">
      <li v-for="port in ports" :key="port">{{ port }}</li>
    </ul>
  </main>
</template>

<style module>
.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>
