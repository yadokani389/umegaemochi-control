<script setup lang="ts">
import * as scanner from '@tauri-apps/plugin-barcode-scanner';
import { Button, Fieldset, useToast } from 'primevue';

const address = defineModel<string>('address', { required: true });
const toast = useToast();

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
}
</script>

<template>
  <Fieldset legend="Scanner" :class="$style.container">
    <Button @click="scanQR">Scan QR</Button>
    <Button @click="scanner.cancel">Cancel QR</Button>
  </Fieldset>
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
