<script setup lang="ts">
import {onMounted, onUnmounted} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

const showedRecords=2;

const books=[
{
    title:"Amintiri din copilarie",
    author:"Ion Luca Caragiale",
    status:"owned",
    language:"ro",
},
    {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    },
    {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    },
    {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    },
    {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    },
    {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    }, {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    }
    , {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    },
    {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    }, {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    },
    {
        title:"Amintiri din copilarie",
        author:"Ion Luca Caragiale",
        status:"owned",
        language:"ro",
    }
]

function handleScroll(event: any) {
    console.log(window.scrollY);
}

onMounted( () => {
    window.addEventListener('scroll', handleScroll
       // this.$refs.el.offsetHeight, this.$refs.el.scrollTop, this.$refs.el.scrollHeight
    );
})

onUnmounted(() => {
    window.removeEventListener('scroll', handleScroll
        // this.$refs.el.offsetHeight, this.$refs.el.scrollTop, this.$refs.el.scrollHeight
    );
})

</script>

<template>
    <div class="container"  ref="el">
        <div class="tableWrap">
            <div class="info">
                <h1>My Library</h1>
                <div class="info">
                    <p>Showing {{showedRecords}} records</p>
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
                <tr v-for="(book, index) in books" >
                    <td>{{index+1}}</td>
                    <td>{{book.title}}</td>
                    <td>{{book.author}}</td>
                    <td>{{book.status}}</td>
                    <td>{{book.language}}</td>
                </tr>
            </table>
        </div>
        </div>
</template>

<style scoped>
.info{
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
}
h1{
    text-align: start;
}
p{
    text-align: end;
}
.tableWrap{
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

.customers td{
    color: #182940;
}

.customers tr:nth-child(even){
    background-color: #f2f2f2 ;
}
.customers tr:nth-child(odd){
    background-color: #b1bdc1 ;
}

.customers tr:hover {background-color: #ddd;}
</style>