<script>
  import { invoke } from "@tauri-apps/api/core";
  import { emit, once } from '@tauri-apps/api/event';

  import Aufgabe from '../aufgabe/Aufgabe.svelte';
  import Liste from '../liste/Liste.svelte';
  import { liste } from '../liste/store.js';

  import File from '../file/File.svelte';

  let aufgabenList = $liste;

  let name = "";
  let greetMsg = "";
  let ausstatung = {
    haupt: 'nichts',
  };
  let list = {};
  let file = {};

  const haupt = ["liste", "suche", "gruppen", "kalendar", "buch", "schließen"];

  // let aufgabenList = [];

  once('file-gewaehlt', async (event) => {
    ausstatung.haupt = 'liste'
    list = await invoke("list", { file });
    $liste = await(invoke("list_alle"));
  })

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
    // invoke("greet", { name });
  }
</script>

<div class="app">
  <header></header>
  <main>
    {#if ausstatung.haupt === 'nichts'}
      <File />
    {:else if ausstatung.haupt === 'liste'}
        <Liste />
    {/if}
  </main>
  <aside>
    <Aufgabe />
  </aside>
  <footer>
    <nav class="kontrollen">
      {#each haupt as item}
        <a href="#">{item}</a>
      {/each}
    </nav>
  </footer>
</div>

<style lang="scss">
:global(.debug) {
  color: #333;
  background-color: #bcd8db;
  // margin: .2em;
  padding: .2em;
  box-sizing: border-box;
}
:global(section.message) {
    padding: .5rem;
}
nav.kontrollen {
  display: flex;
  justify-content: space-around;
  a {
    display: block;
    padding: .5rem;
    padding-bottom: 0;
    color: #333;
    text-decoration: none;
    text-transform: capitalize;
    position: relative;
    &:hover {
      color: #000;
      &::before {
        transform: scaleX(1);
      }
    }
    &::before {
      content: "";
      position: absolute;
      display: block;
      width: 100%;
      height: 2px;
      bottom: 0;
      left: 0;
      background-color: #000;
      transform: scaleX(0);
      transition: transform 0.3s ease;
    }

  }
}
.app {
  display: grid;
  grid-template: "header" "." "main" "." "aside" "." "footer";
  grid-template-rows: 2.8rem 1px 5fr 1px 1fr 1px 2.8rem;
  flex-direction: column;
  background-color: black;
  height: 100vh;
  width: 100vw;
  > aside {
    grid-area: aside;
    background-color: #eee;
  }
  > header {
    grid-area: header;
    background-color: #24c8db;
  }
  > footer {
    grid-area: footer;
    background-color: #fff;
  }
  > main {
    grid-area: main;
    background-color: #f6f6f6;
    overflow-y: auto;
  }
}

.app > main > .container {
  background-color: gray;
}

  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
