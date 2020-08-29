<script>
  import Stats from "../components/Stats.svelte";
  import NewObjects from "../components/NewObjects.svelte";
  import UpdatedObjects from "../components/UpdatedObjects.svelte";
</script>

<style>
  h2 {
    text-align: center;
    font-size: 2em;
    text-transform: uppercase;
    font-weight: 700;
  }

  p {
    margin: 1em auto;
  }
</style>

<svelte:head>
  <title>Aerodome</title>
</svelte:head>

<h2>Aerodome</h2>

<p>
  Welcome to Aerodome, an object repository for the Parallax Propeller!
  <a href="/setup">Click Here to get started.</a>
</p>

<Stats></Stats>

<NewObjects></NewObjects>

<UpdatedObjects></UpdatedObjects>
