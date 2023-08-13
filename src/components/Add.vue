<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const inputTitle = ref("");
const inputAuthor = ref("");
const inputStatus = ref("");
const inputLanguage = ref("");

async function submitBook() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

}
</script>

<template>
  <form class="row">
    <input id="greet-input" v-model="inputTitle" placeholder="Title" />
    <input id="greet-input" v-model="inputAuthor" placeholder="Author" />
    <p>Status:
    <label>Owned
      <input type="radio" value="owned" v-model="inputStatus" />
    </label>Digital
    <label>
      <input type="radio" value="digital" v-model="inputStatus" />
    </label>
    <label>Library
      <input type="radio" value="library" v-model="inputStatus" />
    </label>
    </p>
    <p>Language:
      <label>en
        <input type="radio" value="en" v-model="inputLanguage" />
      </label>
      <label>ro
        <input type="radio" value="ro" v-model="inputLanguage" />
      </label>
    </p>

    <button type="submit" @click="submitBook()">Add Book</button>
  </form>
</template>

<style scoped>
.row {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
}
input, button{
  width:fit-content;
}
</style>
