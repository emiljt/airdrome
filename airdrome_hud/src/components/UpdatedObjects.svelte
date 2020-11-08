<script>
  import { onMount } from "svelte";
  import ObjectCard from "./ObjectCard.svelte";

  let updatedObjects = [];

  /*
   * GET /objects{?limit}{&created,updated,name,targets,languages,keywords,categories}
   */
  function searchObjects(query) {
    return new Promise((resolve, reject) => {
      fetch(`/api/objects?${query}`)
        .then((response) => {
          resolve(response);
        })
        .catch((e) => {
          reject(e);
        });
    });
  }

  onMount(() => {
    // searchObjects("limit=5&order:desc=created")
    //   .then((response) => response.json())
    //   .then((data) => {
    //     results = data;
    //   })
    //   .catch((e) => {
    //     console.log(e);
    //   });
  });
</script>

<style>
  updated-objects {
    display: block;
    padding: 1em 0em 0em 0em;
  }
</style>

<updated-objects>
  <h3>Recently Updated</h3>

  {#each updatedObjects as object}
    <ObjectCard {...object} />
  {/each}
</updated-objects>
