<script>
  import Menu from './Menu.svelte';

  export let segment;

  let displayMenu = false;

  function toggleMenu() {
    displayMenu = !displayMenu;
  }
</script>

<style>
  nav {
    font-weight: 300;
    padding: 0.5em 0.5em 0.5em 0.5em;
  }

  a {
    text-decoration: none;
  }

  form {
    display: inline;
  }

  input[type=search] {
    width: 30%;
    margin: .5em 0em 0em .5em;
    padding: .2em .8em .2em .8em;
    border: darkgray solid 1px;
    border-radius: 1em;
    vertical-align: top;
    transition: all .5s;
  }

  input[type=search]:focus {
    width: 70%;
    transition: all .5s;
  }

  #logo {
    width: 10%;
  }

  #menu-button{
    width: 10%;
    float: right;
  }
</style>

<nav>
  <a href="."><img id="logo" alt="logo" src="logo.svg"></a>

  <form method="get" action="/search">
    <input id="search" name="q" type="search" placeholder="search" on:search={search}>
  </form>

  <span id="menu-button" on:click={toggleMenu}>menu</span>

  {#if displayMenu}
    <Menu bind:display={displayMenu}></Menu>
  {/if}
</nav>
