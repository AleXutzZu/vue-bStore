<script setup lang="ts">
import {computed, Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {fetch} from "@tauri-apps/api/http";

const searchedTerm = ref("");
const authors: Ref<string[]> = ref([])
const imageLink = ref("");
const ISBNBook: Ref<ISBNBook> = ref();

const authorsFormatted = computed<string>(() => {
    if (authors.value.length === 0) return "Loading...";

    let formatted: string = "";
    for (let i = 0; i < authors.value.length; ++i) {
        formatted = formatted + " " + authors.value[i];
        if (i + 1 < authors.value.length) formatted = formatted + ",";
    }
    return formatted;
});

const publishersFormatted = computed<string>(() => {
    let formatted: string = "";
    for (let i = 0; i < ISBNBook.value.publishers.length; ++i) {
        formatted = formatted + " " + ISBNBook.value.publishers[i];
        if (i + 1 < ISBNBook.value.publishers.length) formatted = formatted + ",";
    }
    return formatted;
})

function imageNotFound() {
    alert('That image was not found.');
}

function imageFound() {
    alert('That image is found and loaded');
}

async function getCover() {
    let found = false;
    for (let i = 0; i < ISBNBook.value?.covers.length; i++) {
        var tester = new Image();

        tester.src = "https://covers.openlibrary.org/b/id/" + ISBNBook.value?.covers[i] + "-L.jpg";
        tester.onload = function () {
            found = true;
        }
        if (found) {
            imageLink.value = tester.src;
            return;
        }
        tester.src = "https://covers.openlibrary.org/b/id/" + ISBNBook.value?.covers[i] + "-M.jpg";
        tester.onload = function () {
            found = true;
        }
        if (found) {
            imageLink.value = tester.src;
            return;
        }
        tester.src = "https://covers.openlibrary.org/b/id/" + ISBNBook.value?.covers[i] + "-S.jpg";
        tester.onload = function () {
            found = true;
        }
        if (found) {
            imageLink.value = tester.src;
            return;
        }

    }
}

async function searchBook() {
    try {
        ISBNBook.value = await invoke("search_book", {isbn: searchedTerm.value}) as ISBNBook;
        const responses = await Promise.all(ISBNBook.value?.authors.map(author => fetch(`https://openlibrary.org/${author.key}.json`)));
        authors.value = responses.map(resp => resp.data.name);
    } catch (error) {
        console.log(error);
    }
    // getCover();
}

interface ISBNBook {
    title: string,
    authors: { key: string }[],
    publishers: string[],
    publish_date: string,
    number_of_pages: number,
    covers: number[]
}


</script>

<template>
    <div class="container">
        <h1>Search book by ISBN</h1>
        <div>
            <input v-model="searchedTerm" placeholder="Enter 10 or 13 characters"/>
            <button type="submit" @click="searchBook()" class="specialButton">Search</button>
        </div>
        <div v-if="ISBNBook" class="book">
            <div>
                <h3>Title: {{ ISBNBook.title }}</h3>
                <h3>Author(s): {{ authorsFormatted }}</h3>
                <h3>Number of pages: {{ ISBNBook.number_of_pages }}</h3>
                <h3>Publish date: {{ ISBNBook.publish_date }}</h3>
                <h3>Publisher(s): {{ publishersFormatted }}</h3>
            </div>
            <div>
                <img :src="imageLink" alt="">
            </div>
        </div>

    </div>
</template>

<style scoped>
img {
    width: 200px;
}

h3 {
    text-align: start;
}

.container {
    align-items: flex-start;
    margin: 40px;
    width: 80%;
    min-height: 78vh;
}

button {
    margin: 0 5px;
}

.book {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: fit-content;
    width: 650px;
}
</style>