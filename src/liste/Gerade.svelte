<script>
	import { invoke } from "@tauri-apps/api/core";
	import { format } from "date-fns"; // Assuming date-fns is the internal library for date formatting

	import { liste, aufgabeAendern } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";

	import {
		CaretDoubleDown,
		CaretDoubleUp,
	} from "phosphor-svelte";
	import Circle from 'phosphor-svelte/lib/Circle';



	let aufgaben = $liste;

	let restVerbergen = true;

	function wochentagunabhaengig(aufgaben) {
		return aufgaben.filter(
			(aufgabe) => aufgabe.wochentag === 0,
		);
	}

	function heute(aufgaben) {
		const currentDate = new Date();
		const currentWeekday = (currentDate.getDay() + 6) % 7 + 1; // damit Montag der Tag mit 1 ist usw.
		console.log(currentWeekday)
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

	import { dndzone } from 'svelte-dnd-action';
	import { flip } from 'svelte/animate';

	const flipDurationMs = 100;
	let items = [];
	liste.subscribe((l) => {
		items = wochentagunabhaengig($liste);
		items = items.sort((a, b) => a.position - b.position );
	});
	function handleDndConsider(e) {
		// console.log(e.detail.items);
		items = e.detail.items;
	}
	async function handleDndFinalize(e) {
		console.log(e.detail.items);
		items = e.detail.items;
		let itemsHalb = items.map((i, index) => ({id: i.id, beschreibung: i.beschreibung, position: index }));
		try {
			const res = await invoke('aufgaben_positionieren', { aufgaben: itemsHalb });

			itemsHalb.forEach((a) => {
				let alteAufgabe = $liste.find((aa) => (aa.id === a.id));
				alteAufgabe.position = a.position;
				aufgabeAendern(a.id, alteAufgabe)
			});
		} catch(e) {
			console.error("Await failed:", e)
		}
	}
</script>

{#if $liste.length > 0}
	<div class="liste">
		<header>Heute</header>
		{#each heute($liste) as aufgabe}
			<div
				class="aufgabe"
				class:erledigt={istErledigt(aufgabe)}
				class:gewaehlt={ ($Aufgabe.id === aufgabe.id)}
				on:click={() => aufgabeGewaelt(aufgabe)}
			>
				<div class="satz">
					<div class="extra">
						{#if aufgabe.notiz}<div class="notiz"><Circle weight="fill" size=".8em" /></div>{/if}
						{#if aufgabe.link}<div class="link"><Circle weight="fill" size=".8em"/></div>{/if}
					</div>
					<div class="id">{aufgabe.id}</div>
					<div class="beschreibung">{aufgabe.beschreibung}</div>
				</div>
			</div>
		{/each}
	</div>
	<div class="liste">
		<header>Wochentagunabhängig</header>
		<div use:dndzone={{items, flipDurationMs}} on:consider={handleDndConsider} on:finalize={handleDndFinalize}>
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
					<div class="beschreibung">{aufgabe.beschreibung}</div>
				</div>
			</div>
		{/each}
		</div>
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
				class:gewaehlt={ ($Aufgabe.id === aufgabe.id)}
				on:click={() => aufgabeGewaelt(aufgabe)}
			>
				<div class="satz">
					<div class="id">{aufgabe.id}</div>
					<div class="extra">
						{#if aufgabe.notiz}<div class="notiz"><Circle weight="fill" size=".8em" /></div>{/if}
						{#if aufgabe.link}<div class="link"><Circle weight="fill" size=".8em"/></div>{/if}
					</div>
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
	@import './aufgabe.scss';
	@import './liste.scss';
	.buch {
		display: flex;
		flex-direction: row-reverse;
		> .liste {
			padding: .5rem 0;
			flex: 1;
		}
	}
</style>
