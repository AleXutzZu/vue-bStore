<script setup lang="ts">

</script>

<template>
  <div class="navbarContainer">
      <img src="/src/assets/logo.png" alt="">
  <RouterLink to="/add">ADD</RouterLink>
      <RouterLink to="/">HOME</RouterLink>
  <RouterLink to="/search">SEARCH</RouterLink>
  </div>
</template>

<style scoped>
.navbarContainer{
  border-radius:  0 0 8px 8px;
  border-bottom: 5px solid #347161;
  padding: 0.4em 1.2em;
  font-family: 'Inknut Antiqua', serif;
  background-color: #3ca07b;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  display: flex;
  justify-content: space-around;
  font-size: larger;
  margin: 0;
    align-items: center;
}
img{
    width: 28px;
}
a {
    font-weight: 900;
    color: #182940;
    text-decoration: inherit;
}

a:hover {
    color: 	#0f0f0f!important;
}

a:active{
    color: 	#0f0f0f!important;
}
</style>