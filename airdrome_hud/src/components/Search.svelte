<script>
    import { onMount } from "svelte";
    import ObjectCard from "./ObjectCard.svelte";
    let results = [];

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
        let params = new URLSearchParams(document.location.search);
        let query = params.get("q");
        if (!query) return;
        searchObjects(`name=${query}`)
            .then((response) => response.json())
            .then((data) => {
                results = data;
            })
            .catch((e) => {
                console.log(e);
            });
    });
</script>

<style>
</style>

<search>
    <h1>Search Results</h1>
    {#each results as object}
        <ObjectCard {...object} />
    {/each}
</search>
