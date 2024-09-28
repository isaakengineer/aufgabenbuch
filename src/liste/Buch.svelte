<script>
	import { format } from "date-fns"; // Assuming date-fns is the internal library for date formatting

	import { liste } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";

	let aufgaben = $liste;

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

<div class="buch">
	{#if $liste.length > 0}
		<div class="liste">
			{#each $liste as aufgabe}
				<div
					class="aufgabe"
					class:erledigt={istErledigt(aufgabe)}
					class:gewaehlt={ ($Aufgabe.id === aufgabe.id)}
					on:click={() => aufgabeGewaelt(aufgabe)}
				>
					<div class="satz">
						<div class="id">{aufgabe.id}</div>
						<div class="datum">
							{getHumanReadable(aufgabe).date}
						</div>
						<div class="status">
							{getHumanReadable(aufgabe).status}
						</div>
						<div class="kommentar">{aufgabe.kommentar}</div>
						<div class="beschreibung">{aufgabe.beschreibung}</div>
					</div>

					<!-- {#if import.meta.env.DEV}
	            <div class="dev debug">
	              <div>Gruppe: {aufgabe.gruppe}</div>
	              <div>Notiz: {aufgabe.notiz}</div>
	              <div>Link: <a href={aufgabe.link} target="_blank">{aufgabe.link}</a></div>
	              <div>Wochentag: {aufgabe.wochentag}</div>
	              <div>Priorität: {aufgabe.prioritaet}</div>
	              <div>Position: {aufgabe.position !== null ? aufgabe.position : 'N/A'}</div>
	              <div>Verschoben: {aufgabe.verschoben ? aufgabe.verschoben : 'N/A'}</div>
	              <div>Getan: {aufgabe.getan ? aufgabe.getan : 'N/A'}</div>
	              <div>Vernachlässigt: {aufgabe.vernachlaessigt ? aufgabe.vernachlaessigt : 'N/A'}</div>
	              <div>Kommentar: {aufgabe.kommentar}</div>
	              <div>Erstellt am: {aufgabe.erstellt_an ? aufgabe.erstellt_an : 'N/A'}</div>
	              <div>Geändert am: {aufgabe.geaendert_an ? aufgabe.geaendert_an : 'N/A'}</div>
	            </div>
	          {/if} -->
				</div>
			{/each}
		</div>
	{:else}
		<section class="message">
			<p>Es gibt noch keine erledigte Aufgaben auf Ihre Liste.</p>
			<p>
				Vielleicht versuchen Sie ein Paar Aufgaben auf Ihre Liste
				mal als erledigt zu markieren!
			</p>
		</section>
	{/if}
</div>

<style lang="scss">
	@import "./liste.scss";
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
