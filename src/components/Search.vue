<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
const searchedTerm =ref("");
const imageLink=ref("https://covers.openlibrary.org/b/id/240726-M.jpg");


async function searchBook(){
    let a=await invoke("search_book", {isbn:searchedTerm.value});
    console.log(a);
}

</script>

<template>
<div class="container">
    <h1>Search book by ISBN</h1>
    <div>
    <input v-model="searchedTerm" placeholder="Enter 10 or 13 characters"/>
    <button type="submit" @click="">Search</button>
    </div>
    <div class="book">
        <div>
            <h3>Title: {{}}</h3>
            <h3>Author: {{}}</h3>
            <h3>Number of pages: {{}}</h3>
            <h3>Publish date: {{}}</h3>
            <h3>Publisher: {{}}</h3>
        </div>
        <div>
            <img :src="imageLink" alt="">
        </div>
    </div>

</div>
</template>

<style scoped>
img{
    width: 200px;
}

h3{
    text-align: start;
}
.container{
    align-items: flex-start;
    margin: 40px;
    width: 80%;
    height: fit-content !important;
}
button{
    margin: 0 5px;
}
.book{
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    align-items: center;
}
</style>