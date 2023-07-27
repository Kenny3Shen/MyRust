<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const val = ref();
const name = ref("");

async function myCustomCommand() {
  const a = 0.07;
  const b = 100;
  val.value = await invoke("my_custom_command", { a: a, b: b });
}

async function myCustomCommand2() {
  const a = 0.07;
  const b = 100;
  val.value = await invoke("my_custom_command2", { a: a, b: b });
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
    <button @click="myCustomCommand">Multi</button>
    <button @click="myCustomCommand2">Plus</button>
  </form>

  <p>{{ greetMsg }}</p>
  <p>{{ val }}</p>
</template>
