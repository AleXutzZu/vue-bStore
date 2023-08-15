<script setup lang="ts">
import {computed, Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {fetch} from "@tauri-apps/api/http";

const searchedTerm = ref("");
const authors: Ref<string[]> = ref([])
const imageLink = ref("");
const ISBNBook: Ref<ISBNBook> = ref();
const loaded = ref(true);
const errorMessage = ref("");
const showError = ref(false);


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


async function searchBook() {
    if (!searchedTerm.value.match("/([0-9]{10})/") && !searchedTerm.value.match("/[0-9]{13}/")) {
        showError.value = true;
        errorMessage.value = "Invalid ISBN"
        return;
    }

    loaded.value=false;
    try {
        ISBNBook.value = await invoke("search_book", {isbn: searchedTerm.value}) as ISBNBook;
        loaded.value=true;
        const responses = await Promise.all(ISBNBook.value?.authors.map(author => fetch(`https://openlibrary.org/${author.key}.json`)));
        authors.value = responses.map(resp => resp.data.name);
    } catch (error) {
        console.log(error);
        errorMessage.value = "Could not find any books with that ISBN.";
        showError.value = true;
        loaded.value=true;
    }
    imageLink.value = `https://covers.openlibrary.org/b/id/${ISBNBook.value.covers[0]}-M.jpg`;
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
        <div v-if="!loaded" style="display: flex; justify-content:center; flex-direction: row; width:115%">
            <div class="lds-ring" >
                <br><br><div></div><div></div><div></div><div></div></div>
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

.lds-ring {
    display: inline-block;
    position: relative;
    width: 80px;
    height: 80px;
}
.lds-ring div {
    box-sizing: border-box;
    display: block;
    position: absolute;
    width: 64px;
    height: 64px;
    margin: 8px;
    border: 8px solid #fff;
    border-radius: 50%;
    animation: lds-ring 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
    border-color: #fff transparent transparent transparent;;
}
.lds-ring div:nth-child(1) {
    animation-delay: -0.45s;
}
.lds-ring div:nth-child(2) {
    animation-delay: -0.3s;
}
.lds-ring div:nth-child(3) {
    animation-delay: -0.15s;
}
@keyframes lds-ring {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}
</style>