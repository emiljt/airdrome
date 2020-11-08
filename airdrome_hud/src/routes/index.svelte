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
    padding: 0em 1em 0em 1em;
  }
</style>

<svelte:head>
  <title>Airdrome</title>
</svelte:head>

<h2>Airdrome</h2>

<p>
  Welcome to Airdrome! This is an object repository for the Parallax series of
  microcontrollers, such as the Propeller and Propeller 2. Airdrome provides a
  centeral, easy place to download and manage the objects you need for your next
  project. Airdrome isn't meant to replace the official Parallax OBEX, but
  compliment it. We pull directly from the official OBEX as soon as it's updated
  to bring you the latest (as well as past!) versions of objects.
  <a href="/setup">Click Here to get started.</a>
</p>

<Stats />

<NewObjects />

<UpdatedObjects />
