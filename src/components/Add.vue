<script setup lang="ts">
import {onMounted, onUnmounted, Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Book} from "../main.ts";
import {languages} from "../language";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

const inputTitle = ref("");
const inputAuthor = ref("");
const inputStatus: Ref<string | undefined> = ref(undefined);
const inputLanguage = ref("");
const quote: Ref<Quote | null> = ref(null)
const loadedQuote = ref(true);
const message = ref("");
const showMessage = ref(false);
const isError = ref(false);
const percentageMedium = ref(false);
const percentageSmall = ref(false);

let unsubscribe: UnlistenFn;

function modifyFont() {
    if (!quote.value) return;

    let len = quote.value.quote.length;
    if (len > 230) {
        percentageMedium.value = false;
        percentageSmall.value = true;
    } else if (len > 130) {
        percentageMedium.value = true;
        percentageSmall.value = false;
    } else {
        percentageMedium.value = false;
        percentageSmall.value = false;
    }
}

onMounted(async () => {
    quote.value = await invoke("get_initial_quote") as Quote;
    modifyFont();
    loadedQuote.value = false;
    unsubscribe = await listen("update_quote", event => {
        quote.value = event.payload as Quote;
        modifyFont();
        loadedQuote.value = false;
    })
})

onUnmounted(() => {
    try {
        unsubscribe();
    } catch (ignored) {

    }
})


interface Quote {
    quote: string;
    author: string;
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
    return inputTitle.value && inputAuthor.value && inputLanguage.value && inputStatus.value;
}

</script>

<template>
    <div class="addPage">
        <div class="form">
            <h1>Add a new book</h1>
            <input v-model="inputTitle" placeholder="Title"/>
            <input v-model="inputAuthor" placeholder="Author"/>
            <div class="radioSelector">
                <p style="margin-left: 5px">Status:</p>
                <ul>
                    <li>
                        <label>
                            <input type="radio" value="owned" v-model="inputStatus"/> Owned
                        </label>
                    </li>
                    <li>
                        <label>
                            <input type="radio" value="digital" v-model="inputStatus"/> Digital
                        </label>
                    </li>
                    <li>
                        <label>
                            <input type="radio" value="library" v-model="inputStatus"/> Library
                        </label>
                    </li>

                </ul>
            </div>

            <select v-model="inputLanguage">
                <option value="" disabled selected hidden>Select language</option>
                <option v-for="(key, value)  in languages" :value="value">{{ key }} ({{ value }})</option>
            </select>
            <div class="submitArea">
                <p v-if="showMessage" :class="[{error: isError, success: !isError}, 'errorMessage']">{{ message }}</p>
                <button type="submit" @click="submitBook" class="specialButton">Add book</button>

            </div>
        </div>
        <div class="quote">
            <img src="/src/assets/post-note.png" alt="">
            <div style="z-index: 2; position: absolute; width: 30vw; text-align: center">
                <h2 v-if="loadedQuote">Loading...</h2>
                <div :class="{ h2medium : percentageMedium, h2small: percentageSmall}" v-else-if="quote!=null">
                    <h2>
                        "{{ quote.quote }}"
                    </h2>
                    <h3>
                        - {{ quote.author }}
                    </h3>
                </div>
                <div v-else>
                    Error displaying quote
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
input[type=radio] {
    accent-color: #3c5a64;
}

img {
    width: 300px;
    z-index: 1;
    position: absolute;
}

.addPage {
    display: flex;
    flex-direction: row;
    justify-content: start;
    background-color: #182940;
    height: 91vh;
}

.submitArea {
    width: 90%;
    display: flex;
    flex-direction: row;
    justify-content: end;
    align-items: center;

}

.form {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin: 40px;
    width: 45vw;
}

h2 {
    font-size: 100%;
}

.h2medium {
    font-size: 70% !important;
}

.h2small {
    font-size: 60% !important;
}


.radioSelector {
    display: flex;
    flex-direction: row;
    justify-content: start;
}

li {
    list-style-type: none;
    flex-direction: row;
    display: flex;
    justify-content: start;
}


.quote {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 35vw;
    font-size: larger;
}

.errorMessage {
    width: 20vw;
    margin: 0 7px;
}

.error {
    color: lightpink;
}

.success {
    color: #249b73;
}

</style>
