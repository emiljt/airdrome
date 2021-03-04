<script>
  import { onMount } from "svelte";
  import ObjectCard from "./ObjectCard.svelte";

  let newObjects = [];

  /*
   * GET /objects{?limit}{&created,updated,name,targets,languages,keywords,categories}
   */
  function searchObjects(query) {
    return new Promise((resolve, reject) => {
      fetch(`/objects?${query}`)
        .then((response) => {
          resolve(response);
        })
        .catch((e) => {
          reject(e);
        });
    });
  }

  onMount(() => {
    // searchObjects("limit=5&order:desc=updated")
    //   .then((response) => response.json())
    //   .then((data) => {
    //     results = data;
    //   })
    //   .catch((e) => {
    //     console.log(e);
    //   });
  });
</script>

<new-objects>
  <h3>New Objects</h3>

  {#each newObjects as object}
    <ObjectCard {...object} />
  {/each}
</new-objects>

<style>
  new-objects {
    display: block;
    padding: 1em 0em 0em 0em;
  }
</style>
