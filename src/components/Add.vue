<script setup lang="ts">
import {Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Book} from "../main.ts";
import {languages} from "../language";

const inputTitle = ref("");
const inputAuthor = ref("");
const inputStatus: Ref<string | undefined> = ref(undefined);
const inputLanguage = ref("");

const message = ref("");
const showMessage = ref(false);
const isError = ref(false);

async function submitBook() {
    if (!validate()) {
        message.value = "Inputs are not valid!";
        showMessage.value = true;
        isError.value = true;
        return
    }

    showMessage.value = false;

    const book: Book = {
        title: inputTitle.value,
        author: inputAuthor.value,
        status: inputStatus.value,
        language: inputLanguage.value
    };

    try {
        await invoke("add_book", {...book});
        message.value = "Entry added!";
        showMessage.value = true;
        isError.value = false;
    } catch (error) {
        showMessage.value = true;
        message.value = "An unknown error has occurred. Please try again later"
        isError.value = true;
    }
}

function validate() {
    return inputTitle.value && inputAuthor.value && inputLanguage.value;
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
            <select v-model="inputLanguage">
                <option value="" disabled selected hidden>Select Language</option>
                <option v-for="(key, value)  in languages" :value="value">{{ key }} ({{ value }})</option>
            </select>
            <button type="submit" @click="submitBook">Add Book</button>
            <div v-if="showMessage" :class="{error: isError, success: !isError}">{{ message }}</div>
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
.addPage {
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

.quote {
    display: flex;
    flex-direction: column;
    width: available;
}

.error {
}

.success {
}
</style>
