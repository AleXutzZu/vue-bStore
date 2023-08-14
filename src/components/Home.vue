<script setup lang="ts">

const books=[
{
    title:"Amintiri din copilarie",
    author:"Ion Luca Caragiale",
    status:"owned",
    language:"ro",
}
]
</script>

<template>
    <div class="container">
        <h1>My Library</h1>
        <table class="customers">
            <tr>
                <th>No</th>
                <th>Title</th>
                <th>Author</th>
                <th>Status</th>
                <th>Language</th>
            </tr>
            <tr v-for="(book, index) in books" >
                    <td>{{index+1}}</td>
                    <td>{{book.title}}</td>
                    <td>{{book.author}}</td>
                    <td>{{book.status}}</td>
                    <td>{{book.language}}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped>
.container{
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
    width: 90%;
}

.customers td, .customers th {
    border: 1px solid #ddd;
    padding: 8px;
}

.customers td{
    background-color: #b1bdc1;
    color: #182940;
}

.customers tr:nth-child(even){background-color: #f2f2f2;}

.customers tr:hover {background-color: #ddd;}
</style>