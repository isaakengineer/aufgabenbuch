<script>
	import { format, compareDesc } from "date-fns"; // Assuming date-fns is the internal library for date formatting

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
	const erlediegungsStatus = (a) => {
		let readable = {
			date: null,
			status: "No status available",
		};
		if (a.getan !== null) {
			readable.status = "Getan";
			readable.date = a.getan;
		} else if (a.vernachlaessigt !== null) {
			readable.status = "Vernachlaessigt";
			readable.date = a.vernachlaessigt;
		} else if (a.verschoben !== null) {
			readable.status = "Verschoben";
			readable.date = a.verschoben;
		}
		return readable;
	};
	const nachDatumSortieren = (liste) => {
		let neueListe = liste.sort((a,b) => {
			let a_datum = erlediegungsStatus(a).date;
			let b_datum = erlediegungsStatus(b).date;
			return compareDesc(a_datum, b_datum );
		})
		return neueListe;
	}
</script>

<div class="buch">
	{#if $liste.length > 0}
		<div class="liste">
			{#each nachDatumSortieren($liste) as aufgabe}
				<div
					class="aufgabe"
					class:erledigt={istErledigt(aufgabe)}
					class:gewaehlt={ ($Aufgabe.id === aufgabe.id)}
					on:click={() => aufgabeGewaelt(aufgabe)}
				>
					<div class="satz">
						<div class="id">{aufgabe.id}</div>
						<div class="datum">
							{formatDate(erlediegungsStatus(aufgabe).date)}
						</div>
						<div class="status">
							{erlediegungsStatus(aufgabe).status}
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
	@import './aufgabe.scss';
	.liste {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
