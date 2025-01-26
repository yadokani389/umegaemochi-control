<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import * as scanner from '@tauri-apps/plugin-barcode-scanner';
import { ref } from 'vue';
import { Button, InputText, FloatLabel, DatePicker, InputNumber, Listbox, useToast, Toast } from 'primevue';
import { getAddress, saveAddress } from './utils/cache.ts';

const toast = useToast();

type Settings = {
  weather_city_id: string;
  atcoder_id: string;
  widget_interval: number,
  using_widgets: string[],
};

type DisasterInfo = {
  title: string,
  description: string,
  warning: string,
  occurred: Date,
}

const address = ref<string>("");
const settings = ref<Settings>({ weather_city_id: "", atcoder_id: "", widget_interval: 0, using_widgets: [] });
const disasterInfo = ref<DisasterInfo>({ title: "", description: "", warning: "", occurred: new Date() });
const allWidgets = ref<string[]>([]);

async function scanQR() {
  let permission = await scanner.checkPermissions();
  console.log('Permission:', permission);
  if (permission == 'prompt') {
    permission = await scanner.requestPermissions();
  }

  if (permission == 'denied') {
    console.error('Permission denied');
    toast.add({ severity: 'error', summary: 'Permission denied', detail: 'Camera permission was denied. Please check your settings.', life: 3000 });
    return;
  }

  console.log('Scanning QR code');
  const scanned = await scanner.scan({ windowed: true, formats: [scanner.Format.QRCode] });
  address.value = scanned.content;

  getSettings();
}

function getWidgets() {
  invoke<string[]>('get_widgets', { address: address.value }).then((res) => {
    allWidgets.value = res;
  }).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to get widgets', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}

function getSettings() {
  getWidgets();
  invoke<Settings>('get_settings', { address: address.value }).then((res) => {
    settings.value = res;
    saveAddress(address.value);
  }).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to get settings', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}

function postSettings() {
  invoke('post_settings', { address: address.value, settings: settings.value }).then(() => saveAddress(address.value)).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to post settings', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}

function postDisasterInfo() {
  invoke('post_disaster_info', { address: address.value, info: disasterInfo.value }).then(() => saveAddress(address.value)).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to post disaster info', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}

function clearDisasterInfo() {
  disasterInfo.value = { title: "", description: "", warning: "", occurred: new Date() };
  invoke('clear_disaster_info', { address: address.value }).then(() => saveAddress(address.value)).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to clear disaster info', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}

function scroll(name: string) {
  invoke('scroll', { address: address.value, name }).then(() => saveAddress(address.value)).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to scroll', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}

async function init() {
  address.value = await getAddress();
  console.log('Address:', address.value);
  getSettings();
}

init();
</script>

<template>
  <main :class="$style.container">
    <h1>Welcome to umegaemochi-control</h1>

    <Toast />

    <Button @click="scanQR">Scan QR</Button>
    <Button @click="scanner.cancel">Cancel QR</Button>
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

    <Button @click="postSettings">Post settings</Button>

    <FloatLabel variant="on">
      <InputText v-model="disasterInfo.title" />
      <label>Title</label>
    </FloatLabel>

    <FloatLabel variant="on">
      <InputText v-model="disasterInfo.description" />
      <label>Description</label>
    </FloatLabel>

    <FloatLabel variant="on">
      <InputText v-model="disasterInfo.warning" />
      <label>Warning</label>
    </FloatLabel>

    <DatePicker v-model="disasterInfo.occurred" showTime hourFormat="12" fluid date-format="yy/mm/dd" />

    <Button @click="postDisasterInfo">Post disaster info</Button>
    <Button @click="clearDisasterInfo">Clear disaster info</Button>
    <Button @click="scroll('next')">Scroll up</Button>
    <Button @click="scroll('prev')">Scroll down</Button>
    <div v-for="(widget, index) in settings.using_widgets" :key="index">
      <Button @click="scroll(widget)">Scroll to {{ widget }}</Button>
    </div>
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
