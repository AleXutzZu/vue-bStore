<script setup lang="ts">

</script>

<template>
  <div class="navbarContainer">
  <RouterLink to="/add">Add</RouterLink>
      <RouterLink to="/">Home</RouterLink>
  <RouterLink to="/search">Search</RouterLink>
  </div>
</template>

<style scoped>
.navbarContainer{
  border-radius:  0 0 8px 8px;
  border-bottom: 5px solid #347161;
  padding: 0.6em 1.2em;
  font-family: 'Inknut Antiqua', serif;
  background-color: #3ca07b;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  display: flex;
  justify-content: space-around;
  font-size: larger;
  margin: 0;
}
</style>