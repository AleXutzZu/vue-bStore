<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
const searchedTerm =ref("");
const imageLink=ref("https://covers.openlibrary.org/b/id/240726-M.jpg");

async function addData(){
    await invoke("", {});
}

async function searchBook(){
    let a=await invoke("search_book", {isbn:searchedTerm.value});
    console.log(a);
}

</script>

<template>
<div class="container">
    <button @click="addData()"></button>
    <h1>Search book by ISBN</h1>
    <div>
    <input v-model="searchedTerm" placeholder="Enter the 13-lettered ISBN"/>
    <button type="submit" @click="searchBook()">Search</button>
    </div>
    <div class="book">
        <div>
            <h3>Title: {{}}</h3> //url//title
            <h3>Author: {{}}</h3> //authors
            <h3>Number of pages: {{}}</h3> //authors//number_of_pages
            <h3>Publish date: {{}}</h3> //publish_places//publish_date
            <h3>Publisher: {{}}</h3> //publishers
        </div>
        <div>//cover//medium
            <img :src="imageLink" alt="">
        </div>
    </div>

</div>
</template>

<style scoped>
.container{
    align-items: flex-start;
    margin: 40px;
}
button{
    margin: 0 5px;
}
.book{
    display: flex;
    flex-direction: row;

}
</style>