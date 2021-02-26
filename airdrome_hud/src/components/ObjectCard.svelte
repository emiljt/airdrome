<script>
  export let guid;
  export let name = "[name]";
  export let license = "[license]";
  export let readme = "[readme]";
  export let website = "[website]";
  export let documentation = "[documentation]";
  export let authors = [
    { name: "Unknown", email: "Unknown", website: "Unknown" },
  ];
  export let versions = [
    { guid: "Unknown", number: "Unknown", created: "Unknown" },
  ];
  export let targets = ["Unknown"];
  export let languages = ["Unknown"];
  export let stats = [];
  export let categories = [];
</script>

<object-card>
  <span> <span id="name">{name}</span> <span id="version">v -</span> </span>

  <span>
    <span>Total Downloads: -</span>
    <span>Average Downloads: -</span>
    <span>Updated: -</span>
  </span>

  <ul>
    {#each versions as version}
      <li>
        <a
          href="https://api.airdrome.org/objects/{guid}/versions/{version.guid}/zip"
          >Latest</a
        >
      </li>
    {/each}
  </ul>

  <p>{readme}</p>
</object-card>

<style>
  object-card {
    display: flex;
    flex-flow: row wrap;
    justify-content: space-between;
    border: black solid 1px;
    border-radius: 0.2em;
    margin: 0em 0em 0.8em 1em;
    padding: 0.2em 0.5em 0.1em 0.5em;
    min-height: 3em;
  }

  object-card > span {
    display: flex;
    flex-direction: column;
  }

  object-card > p {
    flex-basis: 100%;
  }

  @media (max-width: 500px) {
    object-card > span:first-child {
      flex-direction: row;
      flex-basis: 100%;
      justify-content: space-between;
      overflow-wrap: everywhere;
    }

    object-card > span:nth-child(2) {
      display: none;
    }

    object-card > p {
      display: none;
    }
  }
</style>
