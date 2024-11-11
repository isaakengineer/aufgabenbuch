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

<div class="erledigen">
	<div class="box">
		<div class="radio-container">
			{#each ['verschoben', 'getan', 'vernachlaessigt'] as status}

				<input type="radio" id={status} name="status" value={status} bind:group={$AufgabeIstErledigt} />
				<label for={status}>{status.charAt(0).toUpperCase() + status.slice(1)}</label>
			{/each}
		</div>

		<input type="text" name="kommentar" bind:value={$Aufgabe.kommentar} placeholder="Kommentar" />
	</div>

	<button class="button" on:click={erledigen}><CheckSquareOffset /></button>
</div>

<style lang="scss">
	@import './radios.scss';
.erledigen {
	display: grid;
	grid-template-columns: 1fr 3rem;
	.box {
		display: flex;
		> input[type="text"] {
			flex-grow: 1;
			margin: .5rem 0px;
			padding: 0px .5rem;

		}
		> .radio-container {
			margin: .5rem;
		}
	}
	> button {
		margin: .5rem;
	}
}
</style>
