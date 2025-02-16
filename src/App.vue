<script setup lang="ts">
import { ref } from 'vue';
import { Toast } from 'primevue';
import { Settings } from './types.ts';
import FieldScanner from './components/FieldScanner.vue';
import FieldSettings from './components/FieldSettings.vue';
import FieldDisasterInfo from './components/FieldDisasterInfo.vue';
import FieldScroll from './components/FieldScroll.vue';
import FieldTodo from './components/FieldTodo.vue';
import { type } from '@tauri-apps/plugin-os';

const address = ref<string>("");
const settings = ref<Settings>({ weather_city_id: "", atcoder_id: "", widget_interval: 0, using_widgets: [], auto_fullscreen: false, auto_hide_cursor: false, using_sports_news: [], using_sound_when_disaster: false });
const osType = type();
const version = __APP_VERSION__;
</script>

<template>
  <main :class="$style.main">
    <Toast />
    <h1>Welcome to umegaemochi-control</h1>
    <div :class="$style.container">
      <FieldScanner v-model:address="address" v-if="['ios', 'android'].includes(osType)" />

      <FieldSettings v-model:address="address" v-model:settings="settings" />

      <FieldDisasterInfo :address="address" />

      <FieldScroll :address="address" :settings="settings" />

      <FieldTodo :address="address" />
    </div>
    <div>Version: v{{ version }}</div>
  </main>
</template>

<style module>
.main {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 10px;
}
</style>
