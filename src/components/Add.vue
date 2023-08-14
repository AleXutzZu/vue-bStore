<script setup lang="ts">
import {Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Book} from "../main.ts";
import {languages} from "../language";

const inputTitle = ref("");
const inputAuthor = ref("");
const inputStatus: Ref<string | undefined> = ref(undefined);
const inputLanguage = ref("");
const quote: Ref<Quote>= ref({})
const loadedQuote = ref(true);
const message = ref("");
const showMessage = ref(false);
const isError = ref(false);

getQuote();


interface Quote{
    quote: string;
    author: string;
}


function getQuote() {
    fetch('https://dummyjson.com/quotes/1')
        .then(res => res.json()).then(res => {
        quote.value = res;
        loadedQuote.value = false;
    });

}


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
            <input v-model="inputTitle" placeholder="Title"/>
            <input v-model="inputAuthor" placeholder="Author"/>
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
            <div class="submitArea">
                <p v-if="showMessage" :class="[{error: isError, success: !isError}, 'errorMessage']">{{ message }}</p>
                <button type="submit" @click="submitBook">Add Book</button>

            </div>
        </div>
        <div class="quote">
            <img src="src/assets/post-note.png" alt="">
            <div style="z-index: 2; position: absolute; width: 30vw; text-align: center">
                <h2 v-if="loadedQuote">Loading...</h2>
                <div v-else>
                    <h2>
                        "{{quote.quote}}"
                    </h2>
                    <h3>
                        -{{ quote.author }}
                    </h3>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
img {
    width: 300px;
    z-index: 1;
    position: absolute;
}

.addPage {
    display: flex;
    flex-direction: row;
    justify-content: start;
}

.submitArea {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: end;
    align-items: end;
}

option {

}

.form {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin: 40px;
    width: 50vw;
}


.quote {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 35vw;
}

.errorMessage {
    width: 25vw;
}

.error {

    color: lightpink;
}

.success {
    color: #249b73;
}
</style>
