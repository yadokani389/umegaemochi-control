<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { Button, Fieldset, Select, useToast } from 'primevue';
import { Settings } from '../types';
import { saveAddress } from '../utils/cache';
import { addToast } from '../utils/misc';
import { ref, watch } from 'vue';

const props = defineProps<{ address: string, settings: Settings }>();
const toast = useToast();
const selectedWidget = ref<string>('');

function scroll(name: string) {
  invoke('scroll', { address: props.address, name }).then(() => saveAddress(props.address)).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to scroll', err);
  });
}

watch(selectedWidget, (value) => {
  if (value) {
    invoke('scroll', { address: props.address, name: value }).then(() => saveAddress(props.address)).catch((err) => {
      console.error(err);
      addToast(toast, 'error', 'Failed to scroll', err);
    });
    selectedWidget.value = '';
  }
});
</script>

<template>
  <Fieldset legend="Scroll">
    <div :class="$style.container">
      <Button @click="scroll('next')">Scroll up</Button>
      <Button @click="scroll('prev')">Scroll down</Button>
      <Select v-model="selectedWidget" :options="settings.using_widgets" placeholder="Scroll to" />
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
