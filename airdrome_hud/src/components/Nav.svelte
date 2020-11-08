<script>
  import Menu from "./Menu.svelte";

  export let segment;

  let displayMenu = false;

  function toggleMenu() {
    displayMenu = !displayMenu;
  }
</script>

<style>
  nav {
    margin: 0 auto;
    font-weight: 300;
    padding: 0.5em 0.5em 0.5em 0.5em;
    max-width: 52em;
  }

  a {
    text-decoration: none;
  }

  form {
    display: inline;
  }

  input[type="search"] {
    width: 30%;
    margin: 0.5em 0em 0em 0.5em;
    padding: 0.2em 0.8em 0.2em 0.8em;
    border: darkgray solid 1px;
    border-radius: 1em;
    vertical-align: top;
    transition: all 0.5s;
  }

  input[type="search"]:focus {
    width: 70%;
    transition: all 0.5s;
  }

  #logo {
    width: 5%;
  }

  #menu-button {
    width: 10%;
    float: right;
  }

  @media (max-width: 500px) {
    #logo {
      width: 10%;
    }
  }
</style>

<nav>
  <a href="."><img id="logo" alt="logo" src="logo.svg" /></a>

  <form method="get" action="/search">
    <input
      id="search"
      name="q"
      type="search"
      placeholder="search"
      on:search={search} />
  </form>

  <span id="menu-button" on:click={toggleMenu}>menu</span>

  {#if displayMenu}
    <Menu bind:display={displayMenu} />
  {/if}
</nav>
