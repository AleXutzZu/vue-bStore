<script setup lang="ts">
import {ref} from "vue";
const searchedTerm =ref("");

</script>

<template>
<div class="container">
    <h1>Search book by ISBN</h1>
    <div>
    <input v-model="searchedTerm" placeholder="Enter search term"/>
    <button type="submit" @click="">Search</button>
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
</style>