<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { Button, Fieldset, useToast } from 'primevue';
import { Settings } from '../types';
import { saveAddress } from '../utils/cache';

const props = defineProps<{ address: string, settings: Settings }>();
const toast = useToast();

function scroll(name: string) {
  invoke('scroll', { address: props.address, name }).then(() => saveAddress(props.address)).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to scroll', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}
</script>

<template>
  <Fieldset legend="Scroll">
    <div :class="$style.container">
      <Button @click="scroll('next')">Scroll up</Button>
      <Button @click="scroll('prev')">Scroll down</Button>
      <div v-for="(widget, index) in settings.using_widgets" :key="index">
        <Button @click="scroll(widget)">Scroll to {{ widget }}</Button>
      </div>
    </div>
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
