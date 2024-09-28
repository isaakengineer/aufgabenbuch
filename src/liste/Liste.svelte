<script>
	import { liste, gruppen } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";
	import { format } from "date-fns";
	import { Ausstattung } from "../routes/store.js";

	import { Empty } from "phosphor-svelte";

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
					<a class="gruppe" class:gewaehlt={g === gruppe} on:click={() => {
							gruppe = g;
							filtern = true;
						}}>
							{#if g === ""}
								<Empty />
							{:else}
								{g}
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
			<div class="liste">
				{#each $liste as aufgabe (aufgabe.id)}
					<div
						class="aufgabe"
						class:erledigt={istErledigt(aufgabe)}
						class:gewaehlt={ ($Aufgabe.id === aufgabe.id)}
						on:click={() => aufgabeGewaelt(aufgabe)}
					>
						<div class="satz">
							<div class="id">{aufgabe.id}</div>
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
	.buch {
		display: flex;
		flex-direction: row-reverse;
		> .liste {
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
	.liste {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.aufgabe {
		opacity: 0.7;
		&:hover {
			opacity: 0.9;
		}
		&.gewaehlt {
			opacity: 1;
		}
		margin: 0.2rem;
		display: flex;

		> .satz {
			padding: 0 0.6rem;
			> div {
				display: inline;
				padding: 0.2rem;
			}
			> .id {
				color: blue;
			}
			> .kommentar {
				color: rgb(6, 6, 100);
			}
		}

		// display: flex;
		// flex-wrap: wrap;
		// gap: .5rem;
		> .beschreibung {
			flex: 1;
		}
		&.erledigt {
			.beschreibung {
				text-decoration: line-through;
			}
		}
		> .dev {
			flex: 1;
			display: flex;
			font-size: 0.9em;
			flex-wrap: wrap;
		}
		// box-shadow: 0px 0px 1px black;
		// padding: .4rem;
		// background-color: #ddd;
		&:hover {
			// background-color: #eee;
			cursor: pointer;
		}
	}
</style>
