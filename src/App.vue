<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import * as scanner from '@tauri-apps/plugin-barcode-scanner';
import { ref } from 'vue';
import { Button, InputText, FloatLabel, DatePicker } from 'primevue';
import { getAddress, saveAddress } from './utils/cache.ts';

type Settings = {
  weather_city_id: string;
  atcoder_id: string;
};

type DisasterInfo = {
  title: string,
  description: string,
  warning: string,
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

function scroll(name: string) {
  invoke('scroll', { address: address.value, name }).then(() => saveAddress(address.value)).catch((err) => {
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
    <DatePicker v-model="disasterInfo.occurred" showTime hourFormat="12" fluid date-format="yy/mm/dd"/>
    <Button @click="postDisasterInfo">Post disaster info</Button>
    <Button @click="scroll('next')">Scroll up</Button>
    <Button @click="scroll('prev')">Scroll down</Button>
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
