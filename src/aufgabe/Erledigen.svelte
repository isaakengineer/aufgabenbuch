<script>
    import { invoke } from "@tauri-apps/api/core";
    import { Aufgabe, AufgabeIstErledigt, updateStore } from './store.js';
	import { liste, addAufgabe, aufgabeAendern } from '../liste/store.js';

    import { CheckSquareOffset } from "phosphor-svelte";

    async function erledigen() {
    	console.log("aufgabe erledigen!")
      console.log($AufgabeIstErledigt);
      if ($AufgabeIstErledigt === 'vernachlaessigt') {
          $Aufgabe.vernachlaessigt = new Date().toISOString();
          $Aufgabe.getan = null;
          $Aufgabe.verschoben = null;
      } else if ($AufgabeIstErledigt === 'getan') {
          $Aufgabe.getan = new Date().toISOString();
          $Aufgabe.vernachlaessigt = null;
          $Aufgabe.verschoben = null;
      } else if ($AufgabeIstErledigt === 'verschoben') {
          $Aufgabe.verschoben = new Date().toISOString();
          $Aufgabe.getan = null;
          $Aufgabe.vernachlaessigt = null;
      }
      let neueAufgabe = await invoke("aufgabe_erledigen", { aufgabe: $Aufgabe });
      $Aufgabe = neueAufgabe;
      aufgabeAendern($Aufgabe.id, neueAufgabe);
    }
</script>

<div class="radio-container">
{#each ['verschoben', 'getan', 'vernachlaessigt'] as status}

    <input type="radio" id={status} name="status" value={status} bind:group={$AufgabeIstErledigt} />
    <label for={status}>{status.charAt(0).toUpperCase() + status.slice(1)}</label>
{/each}
</div>

<input type="text" name="kommentar" bind:value={$Aufgabe.kommentar} placeholder="Kommentar" />

<button class="button" on:click={erledigen}><CheckSquareOffset /></button>

<style lang="scss">
.radio-container {
    display: flex;
    font-size: .9rem;
    gap: 0;
    border: 1px solid #ccc;
    border-radius: .4rem;
    box-shadow: inset 0 1px 3px #ddd;
    > input:first-child + label {
        border-radius: .4rem 0 0 .4rem;
    }
    > label:last-child {
        border-radius: 0 .4rem .4rem 0;
    }
}
input[type="radio"]{
  visibility: hidden;
  height: 0;
  width: 0;
  display: none;
}
label {
    margin: 0;
  display: table-cell;
  vertical-align: middle;
  text-align: center;
  cursor: pointer;
  background-color: #ddd;
  color: #333;
  padding: .2rem .5rem;
  transition: all .3s ease-out;
}
input[type="radio"]:checked + label{
  background-color: #fff;
  color: #000;
  box-shadow: 0 1px 3px #ddd;
}

</style>
