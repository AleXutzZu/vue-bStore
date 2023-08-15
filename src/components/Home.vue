<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

const showedRecords = 14;
const searchedTerm =ref("");
const selectedField =ref("title");
const totalRecords = ref(100);
const currentPage = ref(1);
const totalPages =ref(5);

async function searchBook(){

}

const books = [
    {
        title: "Amfdsasdf ghjhgfdsas dfgbhgfdsdfgb gfdsdfg fdintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    }, {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    }
    , {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    }, {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    },
    {
        title: "Amintiri din copilarie",
        author: "Ion Luca Caragiale",
        status: "owned",
        language: "ro",
    }
]

</script>

<template>
    <div class="container" >
        <div class="tableWrap">
            <h1>My Library</h1>
            <div class="info">
                <input v-model="searchedTerm" placeholder="Enter search term"/>
                <div class="info">
                <p>Search by</p>
                <select v-model="selectedField">
                    <option value="title" >Title</option>
                    <option value="author">Author</option>
                    <option value="status">Status</option>
                    <option value="language">Language</option>
                </select>
                <button type="submit" @click="searchBook()">Go</button>
                </div>
            </div>
            <div class="info">
                <p>Showing {{ showedRecords }} / {{totalRecords}} records </p>
                <div class="info">
                    <img src="/src/assets/arrow1.png" alt="" @click="">
                    <p>Page {{currentPage}} / {{totalPages}}</p>
                    <img src="/src/assets/arrow.png" alt="" @click="">
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
img{
    width: 15px;
    filter: invert(100%) sepia(9%) saturate(0%) hue-rotate(33deg) brightness(109%) contrast(101%);
    margin: 0 10px;
}

img:hover{
    cursor: pointer;
    filter: invert(50%) sepia(9%) saturate(0%) hue-rotate(33deg) brightness(109%) contrast(101%);
    
}
select{
    margin:0 15px ;
}
option{
    color:black;
}
button{
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