<script>
	import { format } from "date-fns"; // Assuming date-fns is the internal library for date formatting

	import { liste } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";

	import {
		CaretDoubleDown,
		CaretDoubleUp,
	} from "phosphor-svelte";

	let aufgaben = $liste;

	let restVerbergen = true;

	function wochentagunabhaengig(aufgaben) {
		return aufgaben.filter(
			(aufgabe) => aufgabe.wochentag === 0,
		);
	}

	function heute(aufgaben) {
		const currentDate = new Date();
		const currentWeekday = currentDate.getDay();
		return aufgaben.filter(
			(aufgabe) => aufgabe.wochentag === currentWeekday,
		);
	}
	function andereTagen(aufgaben) {
		const currentDate = new Date();
		const currentWeekday = currentDate.getDay();
		return aufgaben.filter(
			(aufgabe) =>
				aufgabe.wochentag != currentWeekday &&
				aufgabe.wochentag != 0,
		);
	}

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
			n.map((item, i) => (i === index ? aufgabe : item)),
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
	const formatDate = (date) => {
		return format(new Date(date), "yyyy-MM-dd"); // Format the date as 'day.month.year'
	};
	const getHumanReadable = (a) => {
		let readable = {
			date: "No date available",
			status: "No status available",
		};
		if (a.getan !== null) {
			readable.status = "Getan";
			readable.date = formatDate(a.getan);
		} else if (a.vernachlaessigt !== null) {
			readable.status = "Vernachlaessigt";
			readable.date = formatDate(a.vernachlaessigt);
		} else if (a.verschoben !== null) {
			readable.status = "Verschoben";
			readable.date = formatDate(a.verschoben);
		}
		return readable;
	};
</script>

{#if $liste.length > 0}
	<div class="liste">
		<header>Heute</header>
		{#each heute($liste) as aufgabe}
			<div
				class="aufgabe"
				class:erledigt={istErledigt(aufgabe)}
				on:click={() => aufgabeGewaelt(aufgabe)}
			>
				<div class="satz">
					<div class="id">{aufgabe.id}</div>
					<div class="kommentar">{aufgabe.kommentar}</div>
					<div class="beschreibung">{aufgabe.beschreibung}</div>
				</div>
			</div>
		{/each}
	</div>
	<div class="liste">
		<header>Wochentagunabhängig</header>
		{#each wochentagunabhaengig($liste) as aufgabe}
			<div
				class="aufgabe"
				class:erledigt={istErledigt(aufgabe)}
				on:click={() => aufgabeGewaelt(aufgabe)}
			>
				<div class="satz">
					<div class="id">{aufgabe.id}</div>
					<div class="kommentar">{aufgabe.kommentar}</div>
					<div class="beschreibung">{aufgabe.beschreibung}</div>
				</div>
			</div>
		{/each}
	</div>
	<div class="liste" class:liste-verbergen={restVerbergen}>
		<header class="komplex">
			<div class="titel">Andere Tagen</div>
			{#if andereTagen($liste).length > 0}
				<a
					class="ausweiten"
					on:click={() => (restVerbergen = !restVerbergen)}
				>
					{#if restVerbergen}<CaretDoubleDown
							weight="duotone"
						/>{:else}<CaretDoubleUp weight="duotone" />{/if}
				</a>
			{/if}
		</header>
		{#each andereTagen($liste) as aufgabe}
			<div
				class="aufgabe"
				class:erledigt={istErledigt(aufgabe)}
				on:click={() => aufgabeGewaelt(aufgabe)}
			>
				<div class="satz">
					<div class="id">{aufgabe.id}</div>
					<div class="kommentar">{aufgabe.kommentar}</div>
					<div class="beschreibung">{aufgabe.beschreibung}</div>
				</div>
			</div>
		{/each}
	</div>
{:else}
	<section class="message">
		<p>
			Stellen Sie die Priorität ihre Aufgaben an <code
				>jetzt</code
			>, um sie hier zu sehen.
		</p>
	</section>
{/if}

<style lang="scss">
	.liste {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		> header {
			background-color: #dedede;
			padding: 0.5rem 1rem;
			text-align: end;
			text-decoration: underline;
		}
		> header.komplex {
			display: flex;
			flex-direction: row-reverse;
			justify-content: space-between;
			> .ausweiten {
				text-decoration: none;
				cursor: pointer;
				display: inline-block;
			}
		}
		&.liste-verbergen {
			.aufgabe {
				display: none;
			}
		}
	}
	.aufgabe {
		opacity: 0.7;
		&:hover {
			opacity: 0.9;
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
			> .status {
				background-color: orangered;
				color: white;
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
