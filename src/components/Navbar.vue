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
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-family: 'Inknut Antiqua', serif;
  color: #0f0f0f;
  background-color: #3ca07b;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  display: flex;
  justify-content: space-around;
  font-size: larger;
}
</style>