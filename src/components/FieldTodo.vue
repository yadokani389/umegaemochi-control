<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { Button, Fieldset, Checkbox, InputText, useToast } from 'primevue';
import 'primeicons/primeicons.css';
import { ref } from 'vue';
import { saveAddress } from '../utils/cache';
import { addToast } from '../utils/misc';

type Todo = {
  id: string;
  text: string;
  completed: boolean;
};

const address = defineModel<string>('address', { required: true });
const toast = useToast();
const todos = ref<Todo[]>([]);
const newTodoText = ref<string>('');

async function getTodos() {
  invoke<Todo[]>("get_todos", { address: address.value }).then((res) => {
    todos.value = res.sort((a, b) => a.text.localeCompare(b.text));
    saveAddress(address.value);
  }).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to get todos', err);
  });
}

async function createTodo() {
  if (!newTodoText.value) return;
  invoke('create_todo', { address: address.value, text: newTodoText.value }).then(() => {
    saveAddress(address.value);
    newTodoText.value = '';
    getTodos();
  }).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to create todo', err);
  });
}

async function updateTodo(todo: Todo) {
  invoke('update_todo', { address: address.value, id: todo.id, text: todo.text, completed: todo.completed }).then(() => {
    saveAddress(address.value);
    getTodos();
  }).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to update todo', err);
  });
}

async function deleteTodo(id: string) {
  invoke('delete_todo', { address: address.value, id }).then(() => {
    saveAddress(address.value);
    getTodos();
  }).catch((err) => {
    console.error(err);
    addToast(toast, 'error', 'Failed to delete todo', err);
  });
}
</script>

<template>
  <Fieldset legend="Todo" :class="$style.container">
    <div :class="$style.container">
      <Button @click="getTodos">Get Todos</Button>
      <div v-for="(todo, index) in todos" :key="index" :class="$style.todoItem">
        <Checkbox v-model="todo.completed" :inputId="todo.id" binary @change="updateTodo(todo)" />
        <InputText v-model="todo.text" @blur="updateTodo(todo)" />
        <Button label="Delete" icon="pi pi-times" class="p-button-danger" @click="deleteTodo(todo.id)" />
      </div>
      <div :class="$style.newTodo">
        <InputText v-model="newTodoText" placeholder="New Todo" />
        <Button label="Add" icon="pi pi-plus" @click="createTodo" />
      </div>
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

.todoItem {
  display: flex;
  align-items: center;
}

.todoItem>* {
  margin: 5px;
}

.newTodo {
  display: flex;
  align-items: center;
}

.newTodo>* {
  margin: 5px;
}
</style>
