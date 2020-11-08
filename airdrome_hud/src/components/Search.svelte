<script>
    import { onMount } from "svelte";
    import ObjectCard from "./ObjectCard.svelte";

    let results = [
        {
            guid: "9c7a2502-1a1b-439c-9786-7ac372a4fd20",
            name: "Simple GPS-Module Test Program",
            license: "",
            readme: "",
            website: "",
            documentation: "",
            authors: [],
            versions: [],
            targets: [],
            languages: [],
            stats: [],
            categories: [],
        },
        {
            guid: "410f4baf-6afe-4b24-b919-ea34efac00dd",
            name: "PC Debug Test & Blink LED Demo",
            license: "",
            readme: "",
            website: "",
            documentation: "",
            authors: [],
            versions: [],
            targets: [],
            languages: [],
            stats: [],
            categories: [],
        },
        {
            guid: "8b3a6aae-63db-4b69-a8f8-6cda44a110bb",
            name: "MAX1270 8ch 12b ADC testprogram",
            license: "",
            readme: "",
            website: "",
            documentation: "",
            authors: [],
            versions: [],
            targets: [],
            languages: [],
            stats: [],
            categories: [],
        },
        {
            guid: "f75e4222-511c-4ab2-897f-878f35e4d7e3",
            name: "CANbus Objects -- Simplified Loopback Test",
            license: "",
            readme: "",
            website: "",
            documentation: "",
            authors: [],
            versions: [],
            targets: [],
            languages: [],
            stats: [],
            categories: [],
        },
        {
            guid: "1efe7bc3-da89-4253-8c35-ccffa90a90ea",
            name: "QuickVGA+ Nunchuk Test",
            license: "",
            readme: "",
            website: "",
            documentation: "",
            authors: [],
            versions: [],
            targets: [],
            languages: [],
            stats: [],
            categories: [],
        },
        {
            guid: "cf4d9c27-3bd9-43e4-ac90-36229303260c",
            name: "Automotive Fuel Injector Tester",
            license: "",
            readme: "",
            website: "",
            documentation: "",
            authors: [],
            versions: [],
            targets: [],
            languages: [],
            stats: [],
            categories: [],
        },
    ];

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
