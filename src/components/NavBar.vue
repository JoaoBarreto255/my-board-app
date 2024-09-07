<script setup lang="ts">
</script>

<template>
    <nav class="nav app-nav-setting py-2 px-1">
      <div class="container-fluid">
        <div class="navbar-brand">
          <font-awesome-icon :icon="['fa-solid', 'bars']" />
          My Board
        </div>
      </div>
    </nav>
</template>

<style setup>
.nav.app-nav-setting {
  background: #04AA6D;
}
.app-nav-setting > .container-fluid > .navbar-brand {
  color: azure;
  font-weight: 500;
  font-size: calc(1.075rem + 1.5vw);
  font-family: 'Open Sans',sans-serif;
  text-transform: uppercase;
}
</style>