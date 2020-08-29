<script>
</script>

<style>
  object-card {
    display: flex;
    flex-flow: row wrap;
    justify-content: space-between;
    border: black solid 1px;
    border-radius: 0.2em;
    margin: 0em 0em 0.2em 1em;
    padding: 0em 0.5em 0em 0.5em;
  }

  object-card > span {
    display: flex;
    flex-direction: column;
  }

  object-card > p {
    flex-basis: 100%;
  }

  @media (max-width: 500px) {
    object-card > span:first-child{
      flex-direction: row;
      flex-basis: 100%;
      justify-content: space-between;
    }

    object-card > span:nth-child(2) {
      display: none;
    }

    object-card > p {
      display: none;
    }
  }
</style>

<object-card>
  <span>
    <span id="name">Name of Object</span>
    <span id="version">v0.1.0</span>
  </span>

  <span>
    <span>Total Downloads: 1000</span>
    <span>Average Downloads: 1000</span>
    <span>Updated: 3 months ago</span>
  </span>

  <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
    quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
    consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
    cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
    non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
  </p>
</object-card>
