<script setup lang="ts">
import {ref} from "vue";
const searchedTerm =ref("");

</script>

<template>
<div class="container">
    <h1>Search book</h1>
    <div>
    <input v-model="searchedTerm" placeholder="Enter search term"/>
    <button type="submit" @click="">Add Book</button>
    </div>
</div>
</template>

<style scoped>
.container{
    align-items: center;
}
</style>