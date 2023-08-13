<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Book} from "../main.ts";

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
</script>

<template>
    <div class="row">
        <input id="greet-input" v-model="inputTitle" placeholder="Title"/>
        <input id="greet-input" v-model="inputAuthor" placeholder="Author"/>
        <p>Status:
            <label>Owned
                <input type="radio" value="owned" v-model="inputStatus"/>
            </label>Digital
            <label>
                <input type="radio" value="digital" v-model="inputStatus"/>
            </label>
            <label>Library
                <input type="radio" value="library" v-model="inputStatus"/>
            </label>
        </p>
        <p>Language:
            <label>en
                <input type="radio" value="en" v-model="inputLanguage"/>
            </label>
            <label>ro
                <input type="radio" value="ro" v-model="inputLanguage"/>
            </label>
        </p>

        <button type="submit" @click="submitBook">Add Book</button>
    </div>
</template>

<style scoped>
.row {
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center;
}

input, button {
    width: fit-content;
}
</style>
