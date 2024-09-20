<script>
    import { invoke } from "@tauri-apps/api/core";
    import { Aufgabe, updateStore } from './store.js';

    let selectedOption = null;

  // Determine the initial selected option based on Aufgabe properties
    if ($Aufgabe.vernachlaessigt !== null) {
        selectedOption = 'vernachlaessigt';
    } else if ($Aufgabe.getan !== null) {
        selectedOption = 'getan';
    } else if ($Aufgabe.verschoben !== null) {
        selectedOption = 'verschoben';
    }

    async function erledigen() {
        console.log(selectedOption);
        if (selectedOption === 'vernachlaessigt') {
            $Aufgabe.vernachlaessigt = new Date().toISOString();
        } else if (selectedOption === 'getan') {
            $Aufgabe.getan = new Date().toISOString();
        } else if (selectedOption === 'verschoben') {
            $Aufgabe.verschoben = new Date().toISOString();
        }
        await invoke("aufgabe_erledigen", { aufgabe: $Aufgabe });
    }
</script>

<label>
    Vernachlässigt:
    <input type="radio" name="status" value="vernachlaessigt" bind:group={selectedOption} />
</label>
  
<label>
    Getan:
    <input type="radio" name="status" value="getan" bind:group={selectedOption} />
</label>
  
<label>
    Verschoben:
    <input type="radio" name="status" value="verschoben" bind:group={selectedOption} />
</label>

<input type="text" name="kommentar" bind:value={$Aufgabe.kommentar} placeholder="Kommentar" />

<button on:click={erledigen}>Erledigen</button>