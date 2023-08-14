<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Book} from "../main.ts";
import {languages} from "../language";

const inputTitle = ref("");
const inputAuthor = ref("");
const inputStatus = ref("");
const inputLanguage = ref("");

async function submitBook() {
    const book: Book = {
        title: inputTitle.value,
        author: inputAuthor.value,
        status: inputStatus.value,
        language: inputLanguage.value
    };

    await invoke("add_book", {...book});
}

function selectLanguage(text: String){
  inputLanguage.value=text;
}

</script>

<template>
  <div class="addPage">
    <div class="form">
        <h1>Add a new book</h1>
        <input id="greet-input" v-model="inputTitle" placeholder="Title"/>
        <input id="greet-input" v-model="inputAuthor" placeholder="Author"/>
        <p>Status:
            <label>
                <input type="radio" value="owned" v-model="inputStatus"/>Owned
            </label>
            <label>
                <input type="radio" value="digital" v-model="inputStatus"/>Digital
            </label>
            <label>
                <input type="radio" value="library" v-model="inputStatus"/>Library
            </label>
        </p>
        <select v-model="inputLanguage" >
          <option value="" disabled selected hidden>Select Language</option>
          <option v-for="(key, value)  in languages" value={{value}} >{{key}} ({{value}})</option>
      </select>
        <button type="submit" @click="submitBook">Add Book</button>
    </div>
    <div class="quote">
      <h2>
        " QUOTE "
      </h2>
      <h3>
        - AUTHOR
      </h3>
    </div>
  </div>
</template>

<style scoped>
.addPage{
  display: flex;
  flex-direction: row;

}
.form {
    display: flex;
    justify-content: space-around;
    flex-direction: column;
    align-items: flex-start;
    margin: 40px;
    width: 50vw;
}

input, button, select {
    width: fit-content;
    margin: 10px 0;
}

.quote{
  display: flex;
  flex-direction: column;
  width: available;
}
</style>
