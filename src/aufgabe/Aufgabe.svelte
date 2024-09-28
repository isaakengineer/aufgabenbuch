<script>
	import { Aufgabe, updateStore, AufgabeIstErledigt } from './store.js';
	import { liste, addAufgabe, aufgabeAendern } from '../liste/store.js';
	import { onMount } from 'svelte';
	import { invoke } from "@tauri-apps/api/core";

	import { format } from "date-fns";

	import Erledigen from './Erledigen.svelte';
	import Notiz from './Notiz.svelte';

	import { Aussehen, Ausstattung } from '../routes/store.js';

	import {
		PlusSquare, ArrowCounterClockwise,
		Note, LinkSimple, PencilSimpleLine,
		Command, NotePencil, LinkSimpleHorizontalBreak,
		House, Copy, Browser
	} from "phosphor-svelte";

	export let deaktiviert;

	let formData = {};
	let wochentagOptions = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'];
	let prioritaetOptions = [0, 1, 2, 3, 4];
	let neueAufgabe = {};
	let neueAufgabeId = null;
	let rueckmeldung = false;

	onMount(() => {
		Aufgabe.subscribe(value => {
			formData = value;
		});
	});

	function handleOption(event) {
		const { name, value } = event.target;
		let intValue = parseInt(value, 10);
		updateStore[{ [name]: intValue }]
	}
	function handleChange(event) {

		const { name, value } = event.target;
		console.log(`changing ${name} to ${value}`);
		console.log(typeof(value))
		formData[name] = value;
		updateStore({ [name]: value });
		console.log($Aufgabe)
		console.log(formData)
	}

	let fokus = 'normal';
	if (formData.id === undefined || formData.id === null) {
		fokus = 'normal';
	} else {
		fokus = 'aktionen';
	}
	const resetFormular = () => {
		Aufgabe.reset();
		rueckmeldung = false;
	}

	async function wiederAktivieren () {
		console.log("aufgabe wieder aktivieren", formData.id)
		if ( formData.beschreibung.length == 0 ) {
			rueckmeldung = "Beschreibung darf nicht leer sein!";
		} else {
			let aufgabe = $Aufgabe;
			neueAufgabe = await invoke('aufgabe_wieder_aktivieren', { id: formData.id});
			aufgabeAendern(formData.id, neueAufgabe)
			$Aufgabe = neueAufgabe;
			rueckmeldung = false;
			// TODO: etwas mit listen ändern!
		}
	}

	async function hinfuegen() {
		if ( formData.beschreibung.length == 0 ) {
			rueckmeldung = "Beschreibung darf nicht leer sein!";
		} else {
			console.log("hinfuegen", formData);
			neueAufgabe = await invoke('aufgabe_hinfuegen', { aufgabe: formData });
			addAufgabe(neueAufgabe);
			resetFormular();
		}
	}
	const aendern = async function() {
		if ( formData.beschreibung.length == 0 ) {
			rueckmeldung = "Beschreibung darf nicht leer sein!";
		} else {
			let aufgabe = $Aufgabe;
			neueAufgabe = await invoke('aufgabe_aendern', { aufgabe: aufgabe });
			rueckmeldung = false;
			aufgabeAendern(formData.id, neueAufgabe)
		}
		// TODO: ersetze die alte Aufgabe mit der neuen Aufgabe
	}

	function setFokus(tab) {
		fokus = tab;
	}

	const datumLeserlich = (datumString) => {
		return format( new Date(datumString), "yyyy-MM-dd");
	}
</script>

<form class:deaktiviert={deaktiviert}>
	<div class="tabs-container">
	<div class="tabs">
		<div class={`tab ${fokus === 'normal' ? 'active' : ''}`} on:click={() => setFokus('normal')}>
			<House />
		</div>
		<div class={`tab ${fokus === 'link' ? 'active' : ''}`} on:click={() => setFokus('link')}>
			<LinkSimpleHorizontalBreak />
		</div>
		<div class={`tab ${fokus === 'notiz' ? 'active' : ''}`} on:click={() => setFokus('notiz')}>
			<NotePencil />
		</div>
		<div class={`tab ${fokus === 'aktionen' ? 'active' : ''}`} on:click={() => setFokus('aktionen')}>
			<Command />
		</div>
	</div>
  </div>
  <div class="content">
	{#if rueckmeldung}
		<aside>
		<p>{ rueckmeldung }</p>
	  </aside>
	{/if}
	  <!-- {#if import.meta.env.DEV}
		  <fieldset id="dev">
			  <div>
				  <input type="text" name="gruppe" value={formData.gruppe} disabled placeholder="Gruppe" />
			  </div>
			  <div>
				  <input type="number" name="id" value={formData.id} disabled placeholder="id" size="5"/>
			  </div>
			  <div>
				  <input type="number" name="position" value={formData.position} disabled placeholder="Position" />
			  </div>
			  <label>
				  Erstellt am:
				  <input type="date" name="erstellt_an" value={formData.erstellt_an ? datumLeserlich(formData.erstellt_an) : ''} disabled />
			  </label>

			  <label>
				  Geändert am:
				  <input type="date" name="geaendert_an" value={formData.geaendert_an ? datumLeserlich(formData.geaendert_an) : ''} disabled />
			  </label>
		  </fieldset>
	  {/if} -->
	  {#if fokus != 'notiz'}

				<fieldset id="satz">
			  <textarea name="beschreibung" on:input={handleChange} bind:value={formData.beschreibung} placeholder="Beschreibung"></textarea>
		  </fieldset>

		  <fieldset id="extra">
			  {#if fokus === 'normal'}
					{#if $AufgabeIstErledigt}
						<div>
							<button on:click={wiederAktivieren}>Wieder Aktivieren</button>
						</div>
					{/if}
					<div class="dropdown">
					  <label for="prioritaet">Prioritaet</label>
					  <select name="prioritaet" on:change={handleOption} bind:value={formData.prioritaet} placeholder="Priorität">
						  {#each $Aussehen.optionen.prioritaeten as prioritaet}
							  <option value={prioritaet.id}>{prioritaet.name}</option>
						  {/each}
					  </select>
				  </div>
				  <div class="dropdown">
					  <label for="wochentag">Wochentag</label>
					  <select name="wochentag" on:change={handleOption} bind:value={formData.wochentag} placeholder="Wochentag">
						  {#each $Aussehen.optionen.wochentagen as wochentag}
							  <option value={wochentag.id}>{wochentag.name}</option>
						  {/each}
					  </select>
				  </div>

				  {#if formData.notiz}
					  <div class="dropdown">
						  <label><Note /> Note</label>
							<button><Copy /></button>
					  </div>
				  {/if}
				  {#if formData.link}
						<div class="dropdown">
							<label><LinkSimple /> Link</label>
						  <button><Copy /></button>
							<button><Browser /></button>
					  </div>
				  {/if}
				{:else if fokus === "link"}
					<!-- <fieldset id="link"> -->
						<input type="text" name="link" on:input={handleChange} bind:value={formData.link} placeholder="Link" />
					<!-- </fieldset> -->
			  {:else if fokus === 'aktionen'}
				  <Erledigen />
			  {/if}
		  </fieldset>
		  {:else}
		  <fieldset id="notiz">
			  <Notiz />
			  <button class="close" on:click={() => fokus = 'normal'}>X</button>
		  </fieldset>
	  {/if}
  </div>
  <div class="aktionen">

  	{#if $Aufgabe.id}
		  <button class="icon" on:click={aendern}><PencilSimpleLine /></button>
	  {:else}
		  <button class="icon" on:click={hinfuegen}><PlusSquare /></button>
	  {/if}

  	<button class="icon" on:click={resetFormular}><ArrowCounterClockwise /></button>
  </div>
</form>

<style lang="scss">
:global(button) {
	padding: .2rem .4rem !important;
	// background-color: #222;
}
:global(button.icon) {
	padding: .1rem !important;
	overflow: hidden;
}
:global(.icon > svg) {
	height: 1.2rem;
	width: 1.2rem;
	padding: .2rem .3rem .1rem .3rem;
}
:global(textarea) {
	border-radius: 0;
	border: 0;
}
form.deaktiviert {
	opacity: .5;
	> * {
		pointer-events: none;
	}
}
form > aside {
	padding: 0px 0;
	> p {
		margin: .2rem;
		padding: .2rem 1rem;
	}
}
form {
	> .aktionen {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 1rem 1rem 1rem 0px;
	}
}
form {
	display: grid;
	grid-template-columns: 3rem 1fr 3rem;
	height: 100%;
	padding: 0;
	background-color: #fff;
	border: 0;
}
fieldset {
	margin: 0;
	padding: 0;
	display: flex;
	border: transparent;
	border-radius: 0;
	:global(input) {
		flex: 1;
		padding: .2rem;
	}
	textarea {
		flex: 1;
		padding: .5rem;
	}
	legend {
		font-size: .8rem;
		text-transform: lowercase;
	}
}
#dev {
	background-color: lightgray;
	flex-wrap: wrap;
	gap: .2rem;
	justify-content: space-around;
	font-size: .9rem;
	padding: .2rem;
	margin: .2rem;
	border: 1px solid #777;
}
button.close {
	position: absolute;
	bottom: 0;
	right: 0;
}
fieldset#notiz {
  padding: 0;
  position: relative;
  flex: 1;
  display: flex;
  flex-direction: column;
  > :global(textarea) {
	  flex: 1;
	  // height: 100%;
	  padding: .5rem;
  }
}
fieldset#extra {
	display: flex;
	justify-content: space-between;
}
#actions {
	display: flex;
	justify-content: space-between;
}
.btn-group {
	> button {
		margin: 0;
		border-radius: 0px;
		float: right;
		border: 1px solid black;
	}
}

.tabs-container {
	display: flex;
	flex-direction: column;
	justify-content: space-around;
	background-color: #fff;
	justify-content: space-between;
	:global(> button) {
		margin: .3rem .4rem;
	}
}
.tabs {
	// margin: 1rem 0 1rem .5rem;
	padding: 1rem 0px 1rem 1rem;
  display: flex;
  flex-direction: column;
  cursor: pointer;
  margin-bottom: .1rem;
  gap: .1rem;
  .tab {
		font-size: .9rem;
		padding: .15rem .5rem .15rem .5rem;
		height: fit-content;
		margin-left: .2rem;
		border: 1px solid #ccc;
		border-right: none;
		background: lightgray;
		border-radius: .2rem 0 0 .2rem;
  }
}


.tab.active {
	background: #eee;
	font-weight: bold;
	border-color: #ccc;
}

.content {
	padding: .5rem;
	display: flex;
	flex-direction: column;
	gap: .5rem;
	> fieldset:first-child {
		flex: 1;
	}
}
.content {
  border: 1px solid #ccc;
  border-radius: 5px;
  box-sizing: border-box;
  margin: .4rem;
  margin-left: 0;
  background: #eee;
}



.dropdown {
	> label {
		display: block;
		font-size: .8rem;
		text-align: end;
		color: #333;
	}
}
</style>
