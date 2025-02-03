<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { Button, InputText, FloatLabel, DatePicker, Fieldset, useToast } from 'primevue';
import { saveAddress } from '../utils/cache';

type DisasterInfo = {
  title: string,
  description: string,
  warning: string,
  occurred: Date,
};

const props = defineProps<{ address: string }>();
const disasterInfo = ref<DisasterInfo>({ title: "", description: "", warning: "", occurred: new Date() });
const toast = useToast();

function postDisasterInfo() {
  invoke('post_disaster_info', { address: props.address, info: disasterInfo.value }).then(() => saveAddress(props.address)).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to post disaster info', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}

function clearDisasterInfo() {
  disasterInfo.value = { title: "", description: "", warning: "", occurred: new Date() };
  invoke('clear_disaster_info', { address: props.address }).then(() => saveAddress(props.address)).catch((err) => {
    console.error(err);
    toast.add({ severity: 'error', summary: 'Failed to clear disaster info', detail: err + '\nMake sure both apps are the latest.', life: 3000 });
  });
}
</script>

<template>
  <Fieldset legend="Disaster info">
    <div :class="$style.container">
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
