<script setup lang="ts">
import {onMounted, Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Book} from "../main";

const recordsPerPage = 25;
const offset = ref(0);
const searchedTerm = ref("");
const selectedField = ref("title");
const totalRecords = ref(100);
const currentPage = ref(1);
const totalPages = ref(0);
const books: Ref<Book[] | null> = ref(null);
const loaded = ref(false);
const errorMessage = ref("");
const showError = ref(false);

onMounted(async () => {
    try {
        totalRecords.value = await invoke("book_count", {});
        totalPages.value = Math.trunc(totalRecords.value / recordsPerPage) + (totalRecords.value % recordsPerPage == 0 ? 0 : 1);
        await updateBooks();
    } catch (error) {
        console.log(error);
        showError.value = true;
        errorMessage.value = "Ayaye"
    }
})


function searchBook() {

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

    books.value = await invoke("load_books_interval", {limit: recordsPerPage, offset: offset.value}) as Book[];

    loaded.value = true;
}

</script>

<template>
    <div class="container">
        <div class="tableWrap">
            <h1>My Library</h1>
            <div class="info">
                <input v-model="searchedTerm" placeholder="Enter search term"/>
                <div class="info">
                    <p>Search by</p>
                    <select v-model="selectedField">
                        <option value="title">Title</option>
                        <option value="author">Author</option>
                        <option value="status">Status</option>
                        <option value="language">Language</option>
                    </select>
                    <button type="submit" @click="searchBook()">Go</button>
                </div>
            </div>
            <div class="info">
                <p>Showing {{ recordsPerPage }} / {{ totalRecords }} records </p>
                <div class="info">
                    <img src="/src/assets/arrow_left.png" alt="" @click="prevPage">
                    <p>Page {{ currentPage }} / {{ totalPages }}</p>
                    <img src="/src/assets/arrow_right.png" alt="" @click="nextPage">
                </div>

            </div>

            <table class="customers">
                <tr>
                    <th>No</th>
                    <th>Title</th>
                    <th>Author</th>
                    <th>Status</th>
                    <th>Language</th>
                </tr>
                <tr v-for="(book, index) in books">
                    <td>{{ index + 1 }}</td>
                    <td>{{ book.title }}</td>
                    <td>{{ book.author }}</td>
                    <td>{{ book.status }}</td>
                    <td>{{ book.language }}</td>
                </tr>
            </table>
        </div>
    </div>
</template>

<style scoped>
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
    margin: 0 5px;
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
</style>