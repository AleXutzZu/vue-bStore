<script setup lang="ts">
import {Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
const searchedTerm =ref("");
const Authors=ref([""])
const imageLink=ref("");
const ISBNBook:Ref<ISBNBook | null>=ref(null);

function imageNotFound() {
    alert('That image was not found.');
}

function imageFound() {
    alert('That image is found and loaded');
}

async function getAuthors(){
    Authors.value=[];
    let Fetches=[], JsonFetches=[];
    for (let i = 0; i < ISBNBook.value?.publishers.length; i++)
    {
        Fetches.push("https://openlibrary.org/"+ISBNBook.value?.authors[i].key);
    }
    Fetches=await Promise.all(Fetches);
    for (let i = 0; i < Fetches.length; i++)
    {
        JsonFetches.push(Fetches[i]);
    }
    Authors.value=await Promise.all(JsonFetches);
}
async function getCover(){
    let found =false;
    for(let i=0; i<ISBNBook.value?.covers.length; i++){
        var tester=new Image();

        tester.src="https://covers.openlibrary.org/b/id/"+ISBNBook.value?.covers[i]+"-L.jpg";
        tester.onload = function() {
            found=true;
        }
        if(found){
            imageLink.value=tester.src;
            return;
        }
        tester.src="https://covers.openlibrary.org/b/id/"+ISBNBook.value?.covers[i]+"-M.jpg";
        tester.onload = function() {
            found=true;
        }
        if(found){
            imageLink.value=tester.src;
            return;
        }
        tester.src="https://covers.openlibrary.org/b/id/"+ISBNBook.value?.covers[i]+"-S.jpg";
        tester.onload = function() {
            found=true;
        }
        if(found){
            imageLink.value=tester.src;
            return;
        }

    }
}

async function searchBook(){
    ISBNBook.value=await invoke("search_book", {isbn:searchedTerm.value});
    getAuthors();
    getCover();
}
interface ISBNBook{
    title:string,
    authors: { key:string }[],
    publishers: string[],
    publish_date: string,
    number_of_pages: number,
    covers:number[]
}


</script>

<template>
<div class="container">
    <h1>Search book by ISBN</h1>
    <div>
    <input v-model="searchedTerm" placeholder="Enter 10 or 13 characters"/>
    <button type="submit" @click="searchBook()">Search</button>
    </div>
    <div v-if="ISBNBook" class="book">
        <div>
            <h3>Title: {{ISBNBook.title}}</h3>
            <h3>Author: {{Authors}}</h3>
            <h3>Number of pages: {{ISBNBook.number_of_pages}}</h3>
            <h3>Publish date: {{ISBNBook.publish_date}}</h3>
            <h3>Publisher: {{ISBNBook.publishers}}</h3>
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
    min-height: 78vh;
}
button{
    margin: 0 5px;
}
.book{
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: fit-content;
    width: 650px;
}
</style>