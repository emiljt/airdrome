<script>
  import {goto} from "@sapper/app";

  export let display = "false";

  async function navigate(e) {
      await goto(e.target.href);
      display = false;
  }
</script>

<style>
</style>

<menu>
  <a href="." on:click={navigate}>Home</a>
  <a href="/setup" on:click={navigate}>Setup</a>
  <a href="/search" on:click={navigate}>Search</a>
</menu>
