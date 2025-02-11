<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { Button, InputText, InputNumber, FloatLabel, Fieldset, Listbox, ToggleButton, useToast } from 'primevue';
import { Settings } from '../types.ts';
import { getAddress, saveAddress } from '../utils/cache.ts';
import { addToast, sleep } from '../utils/misc.ts';

const address = defineModel<string>('address', { required: true });
const settings = defineModel<Settings>('settings', { required: true });
const allWidgets = ref<string[]>([]);
const sportsTopics = ref<string[]>([]);

const toast = useToast();

function getWidgets() {
  invoke<string[]>("get_widgets", { address: address.value }).then((res) => {
    allWidgets.value = res;
  }).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to get widgets', err);
  });
}

function getSportsNews() {
  invoke<string[]>("get_sports_news", { address: address.value }).then((res) => {
    sportsTopics.value = res;
  }).catch((err) => {
    console.error(err);
    toast.add({
      severity: "error",
      summary: "Failed to get sports news topics",
      detail: err + "\nMake sure both apps are the latest.",
      life: 3000,
    });
  });
}

function getSettings() {
  getWidgets();
  getSportsNews();
  invoke<Settings>("get_settings", { address: address.value }).then((res) => {
    settings.value = res;
    saveAddress(address.value);
  }).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to get settings', err);
  });
}

function postSettings() {
  invoke('post_settings', { address: address.value, settings: settings.value }).then(() => saveAddress(address.value)).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to post settings', err);
  });
}

async function init() {
  address.value = await getAddress();
  await sleep(1000);
  console.log('Address:', address.value);
  getSettings();
}

init();
</script>

<template>
  <Fieldset legend="Settings">
    <div :class="$style.container">
      <Button @click="getSettings">Get settings</Button>

      <FloatLabel variant="on">
        <InputText v-model="address" />
        <label>Address</label>
      </FloatLabel>

      <FloatLabel variant="on">
        <InputText v-model="settings.weather_city_id" />
        <label>City id</label>
      </FloatLabel>

      <FloatLabel variant="on">
        <InputText v-model="settings.atcoder_id" />
        <label>AtCoder id</label>
      </FloatLabel>

      <FloatLabel variant="on">
        <InputNumber v-model="settings.widget_interval" />
        <label>Widget interval</label>
      </FloatLabel>

      <Listbox v-model="settings.using_widgets" :options="allWidgets" multiple checkmark />
      <Listbox v-model="settings.using_sports_news" :options="sportsTopics" multiple checkmark />
      <ToggleButton v-model="settings.auto_fullscreen" onLabel="Enabled Auto fullscreen"
        offLabel="Disabled Auto fullscreen" />

      <Button @click="postSettings">Post settings</Button>
    </div>
  </Fieldset>
</template>

<style module>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.container>* {
  margin: 5px;
}
</style>
