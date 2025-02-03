<script setup lang="ts">
import { ref } from 'vue';
import { Toast } from 'primevue';
import { Settings } from './types.ts';
import FieldScanner from './components/FieldScanner.vue';
import FieldSettings from './components/FieldSettings.vue';
import FieldDisasterInfo from './components/FieldDisasterInfo.vue';
import FieldScroll from './components/FieldScroll.vue';
import { type } from '@tauri-apps/plugin-os';

const address = ref<string>("");
const settings = ref<Settings>({ weather_city_id: "", atcoder_id: "", widget_interval: 0, using_widgets: [] });
const osType = type();
</script>

<template>
  <main :class="$style.container">
    <h1>Welcome to umegaemochi-control</h1>

    <Toast />

    <FieldScanner v-model:address="address" v-if="['ios', 'android'].includes(osType)" />

    <FieldSettings v-model:address="address" v-model:settings="settings" />

    <FieldDisasterInfo :address="address" />

    <FieldScroll :address="address" :settings="settings" />
  </main>
</template>

<style module>
.container {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.container>* {
  margin: 5px;
}
</style>
