<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import * as scanner from '@tauri-apps/plugin-barcode-scanner';
import { ref } from 'vue';
import { getAddress, saveAddress } from './utils/cache.ts';

type Settings = {
  weather_city_id: String;
  atcoder_id: String;
};

type DisasterInfo = {
  title: String,
  description: String,
  warning: String,
  occurred: Date,
}

let address = ref<string>("");
let settings = ref<Settings>({ weather_city_id: "", atcoder_id: "" });
let disasterInfo = ref<DisasterInfo>({ title: "", description: "", warning: "", occurred: new Date() });

async function scanQR() {
  let permission = await scanner.checkPermissions();
  console.log('Permission:', permission);
  if (permission == 'prompt') {
    permission = await scanner.requestPermissions();
  }

  if (permission == 'denied') {
    console.error('Permission denied');
    return;
  }

  console.log('Scanning QR code');
  const scanned = await scanner.scan({ windowed: true, formats: [scanner.Format.QRCode] });
  address.value = scanned.content;

  getSettings();
}

function getSettings() {
  invoke<Settings>('get_settings', { address: address.value }).then((res) => {
    settings.value = res;
    saveAddress(address.value);
  }).catch((err) => {
    console.error(err);
  });
}

function postSettings() {
  invoke('post_settings', { address: address.value, settings: settings.value }).then(() => saveAddress(address.value)).catch((err) => {
    console.error(err);
  });
}

function postDisasterInfo() {
  invoke('post_disaster_info', { address: address.value, info: disasterInfo.value }).then(() => saveAddress(address.value)).catch((err) => {
    console.error(err);
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

    <button @click="scanQR">Scan QR</button>
    <button @click="scanner.cancel">Cancel QR</button>
    <button @click="getSettings">Get settings</button>
    <input v-model="address" placeholder="0.0.0.0:0000" />
    <input v-model="settings.weather_city_id" placeholder="City id" />
    <input v-model="settings.atcoder_id" placeholder="AtCoder id" />
    <button @click="postSettings">Post settings</button>
    <div>{{ settings }}</div>
    <input v-model="disasterInfo.title" placeholder="Title" />
    <input v-model="disasterInfo.description" placeholder="Description" />
    <input v-model="disasterInfo.warning" placeholder="Warning" />
    <input v-model="disasterInfo.occurred" placeholder="Occurred" />
    <button @click="postDisasterInfo">Post disaster info</button>
    <div>{{ disasterInfo }}</div>
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
