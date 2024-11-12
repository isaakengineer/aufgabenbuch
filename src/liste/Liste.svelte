<script>
	import i18n from '$lib/i18n';
	import { liste, gruppen } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";
	import { format } from "date-fns";
	import { Ausstattung } from "../routes/store.js";

	import { Empty } from "phosphor-svelte";

	import { dndzone } from 'svelte-dnd-action';
	import { flip } from 'svelte/animate';
	import Circle from 'phosphor-svelte/lib/Circle';

	const flipDurationMs = 100;

	let items =[];

		liste.subscribe((value) => {
				items = value;
			});

	function handleSort(e) {
		console.log(e.detail.items);
		 items = e.detail.items;
	}

	let aufgaben = $liste;
	let filtern = false;
	let gruppe = false;

	// Add a new Aufgabe to the liste
	function addAufgabe(aufgabe) {
		liste.update((n) => [...n, aufgabe]);
	}

	// Remove an Aufgabe from the liste by its index
	function removeAufgabe(index) {
		liste.update((n) => n.filter((_, i) => i !== index));
	}

	// Update an existing Aufgabe in the liste
	function updateAufgabe(index, aufgabe) {
		liste.update((n) =>
			n.map((item) => (item.id === index ? aufgabe : item)),
		);
	}
	async function aufgabeGewaelt(aufgabe) {
		$Aufgabe = aufgabe;
	}

	const istErledigt = (a) => {
		return (
			a.getan !== null ||
			a.vernachlaessigt !== null ||
			a.verschoben !== null
		);
	};

	const filterNachGruppe = (liste, g) => {
		console.log(liste)
		return liste.filter((a) => {
			if (gruppe)	return (a.gruppe === g)
			else return (a.gruppe === null)
		})
	};

	const datumLeserlich = (datumString) => {
		return format(new Date(datumString), "yyyy-MM-dd");
	};
</script>

{#if $liste.length > 0}
	<div class="buch">
		{#if $Ausstattung.gruppenZeigen}
			<div class="gruppen">
				{#each $gruppen as g}
					<a class="gruppe" class:gewaehlt={g.name === gruppe} on:click={() => {
							gruppe = g.name;
							filtern = true;
						}}>
							{#if g.name === null}
								<Empty />
							{:else}
								{g.name}
							{/if}
						</a>
				{/each}
			</div>
		{/if}
		{#if filtern && $Ausstattung.gruppenZeigen}
			<div class="liste">
				{#each filterNachGruppe($liste, gruppe) as aufgabe}
					<div
						class="aufgabe"
						class:erledigt={istErledigt(aufgabe)}
						class:gewaehlt={ ($Aufgabe.id === aufgabe.id) }
						on:click={() => aufgabeGewaelt(aufgabe)}
					>
						<div class="satz">
							<div class="id">{aufgabe.id}</div>
							<div class="extra">
								{#if aufgabe.notiz}
									<div class="notiz"><Circle weight="fill" size=".8em" /></div>
								{/if}
								{#if aufgabe.link}
									<div class="link"><Circle weight="fill" size=".8em"/></div>
								{/if}
							</div>
							<div class="beschreibung">
								{aufgabe.beschreibung}
							</div>
							{#if istErledigt(aufgabe)}
								<div class="kommentar">{aufgabe.kommentar}</div>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{:else}
			<div class="liste" use:dndzone={{items, flipDurationMs}} on:consider={handleSort} on:finalize={handleSort}>
				{#each items as aufgabe (aufgabe.id)}
					<div
						class="aufgabe"
						class:erledigt={istErledigt(aufgabe)}
						class:gewaehlt={ ($Aufgabe.id === aufgabe.id)}
						on:click={() => aufgabeGewaelt(aufgabe)}
						animate:flip={{duration: flipDurationMs}}
					>
						<div class="satz">
							<div class="id">{aufgabe.id}</div>
							<div class="extra">
								{#if aufgabe.notiz}<div class="notiz"><Circle weight="fill" size=".8em" /></div>{/if}
								{#if aufgabe.link}<div class="link"><Circle weight="fill" size=".8em"/></div>{/if}
							</div>
							<div class="beschreibung">
								{aufgabe.beschreibung}
							</div>
							{#if istErledigt(aufgabe)}
								<div class="kommentar">{aufgabe.kommentar}</div>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
{:else}
	<section class="message">
		<p>Gerade gibt es keine Aufgaben auf Ihre Liste.</p>
		<p>
			Vielleicht versuchen Sie ein Paar Aufgaben auf Ihre Liste
			hinzufügen, indem Sie die daunten gedrückte Formulare
			nutzen.
		</p>
	</section>
{/if}

<style lang="scss">
	@import './aufgabe.scss';
	@import './liste.scss';
	.buch {
		max-height: 100%;
		overflow: hidden;
		display: flex;
		flex-direction: row-reverse;
		> .liste {
			overflow-y: auto;
			padding: .5rem 0;
			flex: 1;
		}
		> .gruppen {
			padding: .5rem 0;
			width: 5rem;
			display: flex;
			flex-direction: column;
			gap: .5rem;
			> a {
				display: block;
				padding: 0.5rem;
			}
		}
	}
	.gruppen {
		overflow-y: auto;
		> .gruppe {
			background-color: #eee;
			cursor: pointer;
			&:hover {
				background-color: #ccc;
			}
			&.gewaehlt {
				background-color: #333;
				color: #eee;
			}
		}
	}
</style>
