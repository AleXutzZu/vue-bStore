<script setup lang="ts">
import {onMounted, Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Book} from "../main";

const recordsPerPage = 25;
const offset = ref(0);
const searchedTerm = ref("");
const selectedField = ref("Title");
const totalRecords = ref(0);
const currentPage = ref(1);
const totalPages = ref(0);
const books: Ref<Book[] | null> = ref(null);
const loaded = ref(false);
const errorMessage = ref("");
const showError = ref(false);
const appliedFilter = ref(false);

const keywords = ref("");
const filter = ref("");

onMounted(async () => {
    try {
        totalRecords.value = await invoke("book_count", {});
        totalPages.value = Math.trunc(totalRecords.value / recordsPerPage) + (totalRecords.value % recordsPerPage == 0 ? 0 : 1);
        await updateBooks();
        showError.value = false;
    } catch (error) {
        console.log(error);
        showError.value = true;
        errorMessage.value = "Could not access database. You may be offline."
    }
})

async function searchBook() {
    appliedFilter.value = true;

    filter.value = selectedField.value;
    keywords.value = searchedTerm.value;

    try {
        offset.value = 0;
        currentPage.value = 1;
        totalRecords.value = await invoke("filtered_book_count", {
            keywords: searchedTerm.value,
            filter: selectedField.value
        });
        totalPages.value = Math.trunc(totalRecords.value / recordsPerPage) + (totalRecords.value % recordsPerPage == 0 ? 0 : 1);
        await updateBooks();
        showError.value = false;

    } catch (error) {
        //console.log(error)
        errorMessage.value = "Could not access database. You may be offline.";
        showError.value = true;
        appliedFilter.value = false;
    }
}

function nextPage() {
    if (currentPage.value < totalPages.value) {
        currentPage.value++;
        offset.value += recordsPerPage;
        updateBooks();
    }
}

function prevPage() {
    if (currentPage.value > 1) {
        currentPage.value--;
        offset.value -= recordsPerPage;
        updateBooks();
    }
}

async function updateBooks() {
    loaded.value = false;
    try {
        if (appliedFilter.value) {
            books.value = await invoke("load_books_filtered_interval", {
                offset: offset.value,
                limit: recordsPerPage,
                keywords: keywords.value,
                filter: filter.value
            }) as Book[];

        } else {
            books.value = await invoke("load_books_interval", {limit: recordsPerPage, offset: offset.value}) as Book[];
        }
        loaded.value = true;
        showError.value = false;
    } catch (error) {
        errorMessage.value = "Could not access database. You may be offline.";
        showError.value = true;
        loaded.value = true;
    }
}

async function reset() {
    appliedFilter.value = false;
    offset.value = 0;
    currentPage.value = 1;
    totalRecords.value = await invoke("book_count");
    totalPages.value = Math.trunc(totalRecords.value / recordsPerPage) + (totalRecords.value % recordsPerPage == 0 ? 0 : 1);
    await updateBooks();
}

async function removeBook(id: number | undefined) {
    if (!id) return;
    await invoke("remove_book", {id: id});
    offset.value = 0;
    currentPage.value = 1;
    totalRecords.value = await (appliedFilter.value ?
        invoke("filtered_book_count", {
        keywords: keywords.value,
        filter: filter.value
    }) : invoke("book_count"));
    totalPages.value = Math.trunc(totalRecords.value / recordsPerPage) + (totalRecords.value % recordsPerPage == 0 ? 0 : 1);
    await updateBooks();
}

</script>

<template>
    <div class="container">
        <div class="tableWrap">
            <h1>My Library</h1>
            <div class="info">
                <input v-model="searchedTerm" placeholder="Enter search term"/>
                <div class="info">
                    <button type="submit" @click="reset()">Reset filter</button>
                    <p>Search by</p>
                    <select v-model="selectedField">
                        <option value="Title">Title</option>
                        <option value="Author">Author</option>
                        <option value="Status">Status</option>
                        <option value="Language">Language</option>
                    </select>
                    <button type="submit" @click="searchBook()" class="specialButton">Go</button>
                </div>
            </div>
            <div class="info">
                <p>Showing {{ Math.min(recordsPerPage, totalRecords - offset) }} records/page from {{ totalRecords }}
                    records </p>
                <div class="info">
                    <img src="/src/assets/arrow_left.png" alt="" @click="prevPage">
                    <p>Page {{ currentPage }} / {{ totalPages }}</p>
                    <img src="/src/assets/arrow_right.png" alt="" @click="nextPage">
                </div>

            </div>
            <h2 v-if="showError" style="color: lightpink;">
                {{ errorMessage }}
            </h2>
            <div class="lds-ring" v-if="!loaded">
                <br><br>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
            </div>
            <table class="customers" v-else>
                <tr>
                    <th>No.</th>
                    <th>Title</th>
                    <th>Author</th>
                    <th>Status</th>
                    <th>Language</th>
                    <th>Delete</th>
                </tr>
                <tr v-for="(book, index) in books">
                    <td>{{ index + offset + 1 }}</td>
                    <td>{{ book.title }}</td>
                    <td>{{ book.author }}</td>
                    <td>{{ book.status }}</td>
                    <td>{{ book.language }}</td>
                    <td>
                        <button class="x" @click="removeBook(book.id)">X</button>
                    </td>
                </tr>
            </table>
            <br>
            <br>
        </div>
    </div>
</template>

<style scoped>
.x {
    padding: 0.3em 0.6em;
    color: #e8e8e8;
}

.x:active {
    border-color: #182940;
    color: #182940;
    background-color: #e8e8e8;
}

img {
    width: 15px;
    filter: invert(100%) sepia(9%) saturate(0%) hue-rotate(33deg) brightness(109%) contrast(101%);
    margin: 0 10px;
}

img:hover {
    cursor: pointer;
    filter: invert(50%) sepia(9%) saturate(0%) hue-rotate(33deg) brightness(109%) contrast(101%);

}


select {
    margin: 0 15px;
}

option {
    color: black;
}

button {
    margin: 0 15px;
}

.info {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
}

h1 {
    text-align: start;
}

p {
    text-align: end;
}

.tableWrap {
    justify-content: start;
    align-items: start;
    margin: 0 40px;
}

.customers th {
    padding-top: 12px;
    padding-bottom: 12px;
    text-align: center;
    background-color: #3c5a64;
    color: white;
}

.customers {
    font-family: Arial, Helvetica, sans-serif;
    border-collapse: collapse;
    width: 100%;
}

.customers td, .customers th {
    border: 1px solid #ddd;
    padding: 8px;
}

.customers td {
    color: #182940;
}

.customers tr:nth-child(even) {
    background-color: #f2f2f2;
}

.customers tr:nth-child(odd) {
    background-color: #b1bdc1;
}

.customers tr:hover {
    background-color: #ddd;
}

.container {
    min-height: 100vh;
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