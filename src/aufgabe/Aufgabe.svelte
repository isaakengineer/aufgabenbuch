<script>
	import i18n from "$lib/i18n";
	import { Aufgabe, updateStore, AufgabeIstErledigt } from "./store.js";
	import { liste, addAufgabe, aufgabeAendern } from "../liste/store.js";
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";

	import { format } from "date-fns";

	import Erledigen from "./Erledigen.svelte";
	import Notiz from "./Notiz.svelte";

	import { Aussehen, Ausstattung } from "../routes/store.js";

	import PlusSquare from "phosphor-svelte/lib/PlusSquare";
	import ArrowCounterClockwise from "phosphor-svelte/lib/ArrowCounterClockwise";
	import Note from "phosphor-svelte/lib/Note";
	import LinkSimple from "phosphor-svelte/lib/LinkSimple";
	import PencilSimpleLine from "phosphor-svelte/lib/PencilSimpleLine";
	import Command from "phosphor-svelte/lib/Command";
	import NotePencil from "phosphor-svelte/lib/NotePencil";
	import LinkSimpleHorizontalBreak from "phosphor-svelte/lib/LinkSimpleHorizontalBreak";
	import House from "phosphor-svelte/lib/House";
	import Copy from "phosphor-svelte/lib/Copy";
	import Browser from "phosphor-svelte/lib/Browser";
	import Empty from "phosphor-svelte/lib/Empty";
	import ArrowsVertical from "phosphor-svelte/lib/ArrowsVertical";
	import ArrowsInLineVertical from "phosphor-svelte/lib/ArrowsInLineVertical";

	export let deaktiviert;

	let wochentagOptions = [
		"Montag",
		"Dienstag",
		"Mittwoch",
		"Donnerstag",
		"Freitag",
		"Samstag",
		"Sonntag",
	];
	let prioritaetOptions = [0, 1, 2, 3, 4];
	let neueAufgabe = {};
	let neueAufgabeId = null;
	let rueckmeldung = false;

	let fokus = "normal";
	if ($Aufgabe.id === undefined || $Aufgabe.id === null) {
		fokus = "normal";
	} else {
		fokus = "aktionen";
	}
	const resetFormular = () => {
		Aufgabe.reset();
		rueckmeldung = false;
		fokus = "normal";
		isAsideExpanded = false;
	};

	async function hinfuegen() {
		if ($Aufgabe.beschreibung.length == 0) {
			rueckmeldung = "Beschreibung darf nicht leer sein!";
		} else {
			neueAufgabe = await invoke("aufgabe_hinfuegen", {
				aufgabe: $Aufgabe,
			});
			addAufgabe(neueAufgabe);
			resetFormular();
		}
	}
	const aendern = async function () {
		if ($Aufgabe.beschreibung.length == 0) {
			rueckmeldung = "Beschreibung darf nicht leer sein!";
		} else {
			let aufgabe = $Aufgabe;
			neueAufgabe = await invoke("aufgabe_aendern", { aufgabe: aufgabe });
			rueckmeldung = false;
			aufgabeAendern($Aufgabe.id, neueAufgabe);
		}
		// TODO: ersetze die alte Aufgabe mit der neuen Aufgabe
	};

	function setFokus(tab) {
		fokus = tab;
	}

	const datumLeserlich = (datumString) => {
		return format(new Date(datumString), "yyyy-MM-dd");
	};

	let isAsideExpanded = false;

	function toggleAsideHeight() {
		isAsideExpanded = !isAsideExpanded;
		fokus = fokus == "notiz" ? "normal" : "notiz";
	}
</script>

<form
	class:deaktiviert
	class:expanded={isAsideExpanded}
	class:erledigt={$Aufgabe.id}
>
	<div class="tabs-container">
		<div class="tabs">
			<!-- <div class={`tab ${$Aufgabe.id ? 'active' : ''}`}> -->
			<div class="tab">
				{#if $Aufgabe.hasOwnProperty("id") && $Aufgabe.id}
					{$Aufgabe.id}
				{:else}
					<Empty />
				{/if}
			</div>
			<div class="tab">
				<button on:click={toggleAsideHeight}>
					{#if isAsideExpanded}
						<ArrowsInLineVertical />
					{:else}
						<ArrowsVertical />
					{/if}
				</button>
			</div>
			{#if fokus == "notiz"}
				<div class="tab">
					<div class="aktionen">
						{#if $Aufgabe.id}
							<button class="icon" on:click={aendern}
								><PencilSimpleLine /></button
							>
						{:else}
							<button class="icon" on:click={hinfuegen}
								><PlusSquare /></button
							>
						{/if}

						<button class="icon" on:click={resetFormular}
							><ArrowCounterClockwise /></button
						>
					</div>
				</div>
			{/if}
			<!-- <div
				class={`tab ${fokus === "normal" ? "active" : ""}`}
				on:click={() => setFokus("normal")}
			>
				<House />
			</div> -->
			<!-- <div class={`tab ${fokus === 'link' ? 'active' : ''}`} on:click={() => setFokus('link')}>
			<LinkSimpleHorizontalBreak />
		</div> -->
			<!-- <div
				class={`tab ${fokus === "notiz" ? "active" : ""}`}
				on:click={() => setFokus("notiz")}
			>
				<NotePencil />
			</div> -->
			<!--<div class={`tab ${fokus === 'aktionen' ? 'active' : ''}`} on:click={() => setFokus('aktionen')}>
			<Command />
		</div>-->
		</div>
	</div>
	<div class="content">
		{#if rueckmeldung}
			<aside>
				<p>{rueckmeldung}</p>
			</aside>
		{/if}
		<!-- {#if import.meta.env.DEV}
			<fieldset id="dev">
				<div>
					<input type="text" name="gruppe" value={$Aufgabe.gruppe} disabled placeholder="Gruppe" />
				</div>
				<div>
					<input type="number" name="id" value={$Aufgabe.id} disabled placeholder="id" size="5"/>
				</div>
				<div>
					<input type="number" name="position" value={$Aufgabe.position} disabled placeholder="Position" />
				</div>
				<label>
					Erstellt am:
					<input type="date" name="erstellt_an" value={$Aufgabe.erstellt_an ? datumLeserlich($Aufgabe.erstellt_an) : ''} disabled />
				</label>

				<label>
					Geändert am:
					<input type="date" name="geaendert_an" value={$Aufgabe.geaendert_an ? datumLeserlich($Aufgabe.geaendert_an) : ''} disabled />
				</label>
			</fieldset>
		{/if} -->
		{#if fokus != "notiz"}
			<fieldset id="satz">
				<textarea
					name="beschreibung"
					bind:value={$Aufgabe.beschreibung}
					placeholder={$i18n.t("aufgabe.beschreibung")}
				></textarea>
			</fieldset>
		{:else}
			<fieldset id="extra">
				<div class="seiten">
					<div
						class="radio-container senkrecht"
						style="grid-template-columns: repeat(6, 1.6rem);"
					>
						{#each $Aussehen.optionen.prioritaeten as prioritaet}
							<input
								type="radio"
								id={"prioritaet" + prioritaet.id}
								name="prioritaet"
								value={prioritaet.id}
								bind:group={$Aufgabe.prioritaet}
							/>
							<label for={"prioritaet" + prioritaet.id}>
								<div class="label">
									{$i18n.t(
										"priorität-optionen." + prioritaet.id,
									)}
								</div>
							</label>
						{/each}
					</div>
					<!-- <select name="prioritaet" on:change={handleOption} bind:value={$Aufgabe.prioritaet} placeholder="Priorität">
					{#each $Aussehen.optionen.prioritaeten as prioritaet}
						<option value={prioritaet.id}>{prioritaet.name}</option>
					{/each}
				</select> -->
				</div>
				<div class="seiten">
					<div
						class="radio-container senkrecht"
						style="grid-template-columns: repeat(8, 1.6rem);"
					>
						<!-- <label for="wochentag">
							{$i18n.t("aufgabe.wochentag")}
						</label> -->
						{#each $Aussehen.optionen.wochentagen as wochentag}
							<input
								type="radio"
								id={"prioritaet" + wochentag.id}
								name="wochentag"
								value={wochentag.id}
								bind:group={$Aufgabe.wochentag}
							/>
							<label for={"wochentag" + wochentag.id}>
								<div class="label">
									{$i18n.t(
										"wochentag-optionen." + wochentag.id,
									)}
								</div>
							</label>
						{/each}
					</div>
					<!-- <select name="prioritaet" on:change={handleOption} bind:value={$Aufgabe.prioritaet} placeholder="Priorität">
					{#each $Aussehen.optionen.prioritaeten as prioritaet}
						<option value={prioritaet.id}>{prioritaet.name}</option>
					{/each}
				</select> -->
				</div>
				<!-- <div class="dropdown">
					<label for="wochentag">
						{$i18n.t("aufgabe.wochentag")}
					</label>
					<select
						name="wochentag"
						bind:value={$Aufgabe.wochentag}
						placeholder={$i18n.t("aufgabe.wochentag")}
					>
						{#each $Aussehen.optionen.wochentagen as wochentag}
							<option value={wochentag.id}>
								{$i18n.t(
									"wochentag-optionen." +
										wochentag.id.toString(),
								)}
							</option>
						{/each}
					</select>
				</div> -->

				<!-- !!TODO!! -->
				<!-- {#if $Aufgabe.notiz}
				<div class="dropdown">
					<label><Note /> Note</label>
					<button><Copy /></button>
				</div>
			{/if}
			{#if $Aufgabe.link}
				<div class="dropdown">
					<label><LinkSimple /> Link</label>
					<button><Copy /></button>
					<button><Browser /></button>
				</div>
			{/if} -->
			</fieldset>
			<fieldset id="notiz">
				<Notiz />
				<!-- <button class="close" on:click={() => fokus = 'normal'}>X</button> -->
			</fieldset>
			<fieldset id="link">
				<input
					type="text"
					name="link"
					bind:value={$Aufgabe.link}
					placeholder={$i18n.t("aufgabe.link")}
				/>
			</fieldset>
		{/if}
	</div>
	{#if fokus != "notiz"}
		<div class="aktionen">
			{#if $Aufgabe.id}
				<button class="icon" on:click={aendern}
					><PencilSimpleLine /></button
				>
			{:else}
				<button class="icon" on:click={hinfuegen}><PlusSquare /></button
				>
			{/if}

			<button class="icon" on:click={resetFormular}
				><ArrowCounterClockwise /></button
			>
		</div>
	{/if}
	{#if $Aufgabe.id}
		<Erledigen />
	{/if}
</form>

<style lang="scss">
	@import "./radios.scss";
	textarea {
		resize: none;
		height: auto;
		font-size: 1rem;
	}
	:global(button) {
		padding: 0.2rem 0.4rem !important;
		// background-color: #222;
	}
	:global(button.icon) {
		padding: 0.05rem !important;
		overflow: hidden;
		text-align: center;
		display: flex;
	}
	:global(.icon > svg) {
		height: 1.2rem;
		width: 1.2rem;
		padding: 0.2rem 0.3rem 0.1rem 0.3rem;
	}
	:global(textarea) {
		border-radius: 0;
		border: 0;
	}
	form.deaktiviert {
		opacity: 0.5;
		> * {
			pointer-events: none;
		}
	}
	form > aside {
		padding: 0px 0;
		> p {
			margin: 0.2rem;
			padding: 0.2rem 1rem;
		}
	}
	form {
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
			padding: 0.2rem;
		}
		textarea {
			flex: 1;
			padding: 0.5rem;
		}
		legend {
			font-size: 0.8rem;
			text-transform: lowercase;
		}
	}
	#dev {
		background-color: lightgray;
		flex-wrap: wrap;
		gap: 0.2rem;
		justify-content: space-around;
		font-size: 0.9rem;
		padding: 0.2rem;
		margin: 0.2rem;
		border: 1px solid #777;
	}
	button.close {
		position: absolute;
		bottom: 0;
		right: 0;
	}
	fieldset#notiz {
		flex: 1;
		display: flex;
		flex-direction: column;
		> :global(textarea) {
			flex: 1;
			// height: 100%;
			padding: 0.5rem;
			margin: 0px 0.5rem;
			font-size: 1rem;
		}
	}
	fieldset#extra {
		display: flex;
		justify-content: space-between;
		flex-grow: 0;
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
	form {
		display: grid;
		grid-template-columns: 1fr 2.3rem 3rem;
		grid-template-areas: "content tabs aktionen";
		grid-template-rows: 1fr;
		height: 5rem !important;
		&.expanded {
			grid-template-areas:
				"content content content"
				"tabs tabs tabs";
			grid-template-rows: 1fr 2.5rem;
		}
		&.erledigt {
			height: 8rem !important;
			grid-template-areas:
				"content tabs aktionen"
				"erledigen erledigen erledigen";
			grid-template-rows: 1fr 3rem;
			&.expanded {
				grid-template-areas:
					"content content content"
					"tabs tabs tabs"
					"erledigen erledigen erledigen";
				grid-template-rows: 1fr 2.5rem 3rem;
			}
		}
		transition: height 0.3s ease;
		&.expanded {
			height: 75vh !important;
		}
		textarea,
		input[type="text"] {
			box-sizing: border-box;
			font-size: 1rem;
			padding: 0.5rem 0.5rem;
			margin: 0.5rem;
		}
		gap: 0px;
		// background-color: blue;
		> .content {
			grid-area: content;
			margin: 0px;
			border-radius: 0;
			border-right: none;
			box-sizing: border-box;
			padding: 0px;
			// background-color: yellow;
		}
		> .tabs-container {
			grid-area: tabs;
			padding: 0px;
			margin: 0px;
			// background-color: red;
		}
		.aktionen {
			grid-area: aktionen;
			display: flex;
			flex-direction: column;
			justify-content: space-evenly;
			padding: 0px 0.5rem;
		}
		:global(.erledigen) {
			grid-area: erledigen;
		}
	}
	form.expanded .aktionen {
		flex-direction: row;
		justify-content: space-evenly;
		width: 100%;
	}
	form .tabs-container {
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		background-color: #fff;
		justify-content: space-between;
		:global(> button) {
			margin: 0.3rem 0.4rem;
		}
	}

	.tabs {
		// padding: 1rem 0px 1rem 1rem;
		// display: flex;
		// flex-direction: column;
		// margin-bottom: .1rem;
		// gap: .1rem;
		// padding-right: 1rem;
		/* margin-left: -3px; */
		/* margin-right: auto; */

		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		.tab {
			box-sizing: border-box;
			margin: 0px;
			padding: 0.05rem;
			min-height: 2rem;
			flex-grow: 1;
			text-align: center;
			justify-content: center;
			align-items: center;
			display: flex;
			cursor: pointer;
			font-size: 0.9rem;
			border: 1px solid #ccc;

			background: lightgray;
			// border-radius: .2rem 0 0 .2rem;
			&.active {
				background: #eee;
				border-color: #ccc;
				border-left: none;
			}
		}
	}
	form.expanded .tabs {
		flex-direction: row;
	}

	.content {
		padding: 0.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		> fieldset:first-child {
			flex: 1;
		}
	}
	.content {
		border: 1px solid #ccc;
		border-radius: 5px;
		box-sizing: border-box;
		margin: 0.4rem;
		margin-left: 0;
		background: #eee;
	}
	.dropdown {
		> label {
			display: block;
			font-size: 0.8rem;
			text-align: end;
			color: #333;
		}
	}
	#satz {
		height: 3rem;
	}
</style>
