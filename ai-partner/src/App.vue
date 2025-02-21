<script setup lang="ts">
import Layout from './layout/index.vue'
import NaiveProvider from './components/Provider/NaiveProvider.vue';


</script>

<template>
  <NaiveProvider>
    <Layout/>
  </NaiveProvider>
</template>

<style scoped>


</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
}


.row {
  display: flex;
  justify-content: center;
}
body{
  margin: 0;
}
a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border: none;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  transition: border-color 0.25s;
}

input,
button {
  outline: none;
}

.copy-button {
  position: absolute;
  top: 0.5em;
  right: 0.5em;
  padding: 0.25em 0.5em;
  background-color: #f5f5f5;
  border: 1px solid #ccc;
  border-radius: 3px;
  cursor: pointer;
  font-size: 0.8em;
  color: #333;
}

.copy-button:hover {
  background-color: #eaeaea;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>