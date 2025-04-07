<script>
	import i18n from "$lib/i18n";
	import { invoke } from "@tauri-apps/api/core";
	import { Aufgabe, AufgabeIstErledigt, updateStore } from "./store.js";
	import { liste, addAufgabe, aufgabeAendern } from "../liste/store.js";

	import CheckSquareOffset from "phosphor-svelte/lib/CheckSquareOffset";

	let rueckmeldung = "";

	async function wiederAktivieren() {
		let neueAufgabe;

		if ($Aufgabe.beschreibung.length == 0) {
			rueckmeldung = "Beschreibung darf nicht leer sein!";
		} else {
			let aufgabe = $Aufgabe;
			neueAufgabe = await invoke("aufgabe_wieder_aktivieren", {
				id: $Aufgabe.id,
			});
			aufgabeAendern($Aufgabe.id, neueAufgabe);
			$Aufgabe = neueAufgabe;
			rueckmeldung = false;
			// TODO: etwas mit listen ändern!
		}
	}

	async function erledigen() {
		console.log("aufgabe erledigen!");
		console.log($AufgabeIstErledigt);
		if ($AufgabeIstErledigt === "vernachlaessigt") {
			$Aufgabe.vernachlaessigt = new Date().toISOString();
			$Aufgabe.getan = null;
			$Aufgabe.verschoben = null;
		} else if ($AufgabeIstErledigt === "getan") {
			$Aufgabe.getan = new Date().toISOString();
			$Aufgabe.vernachlaessigt = null;
			$Aufgabe.verschoben = null;
		} else if ($AufgabeIstErledigt === "verschoben") {
			$Aufgabe.verschoben = new Date().toISOString();
			$Aufgabe.getan = null;
			$Aufgabe.vernachlaessigt = null;
		}
		let neueAufgabe = await invoke("aufgabe_erledigen", {
			aufgabe: $Aufgabe,
		});
		$Aufgabe = neueAufgabe;
		aufgabeAendern($Aufgabe.id, neueAufgabe);
	}
</script>

<div class="erledigen">
	<div class="box">
		<div class="radio-container">
			{#each ["getan", "vernachlaessigt", "verschoben"] as status}
				<input
					type="radio"
					id={status}
					name="status"
					value={status}
					bind:group={$AufgabeIstErledigt}
				/>
				<label for={status}>
					{$i18n.t("aufgabe.erledigt-optionen." + status)}
				</label>
			{/each}
		</div>
		<div>
			{#if $AufgabeIstErledigt}
				<div>
					<button on:click={wiederAktivieren}>
						{$i18n.t("aufgabe.erledigt.rückgängig")}
					</button>
				</div>
			{/if}
		</div>
		{#if rueckmeldung}
			<div class="error">{rueckmeldung}</div>
		{/if}

		<input
			type="text"
			name="kommentar"
			bind:value={$Aufgabe.kommentar}
			placeholder={$i18n.t("aufgabe.kommentar")}
		/>
	</div>

	<button class="button" on:click={erledigen}><CheckSquareOffset /></button>
</div>

<style lang="scss">
	@import "./radios.scss";
	.erledigen {
		display: grid;
		grid-template-columns: 1fr 3rem;
		.box {
			display: flex;
			> input[type="text"] {
				flex-grow: 1;
				margin: 0.5rem 0px;
				padding: 0px 0.5rem;
			}
			> .radio-container {
				margin: 0.5rem;
			}
		}
		> button {
			margin: 0.5rem;
		}
	}
</style>
