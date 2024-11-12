<script>
	import { format } from "date-fns";

	import { invoke } from "@tauri-apps/api/core";

	import i18n from '$lib/i18n';

	import { liste, aufgabeAendern } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";
	import AufgabeBox from './AufgabeBox.svelte';

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
		<header>{ $i18n.t('abbild.gerade.header-heute') }</header>
		{#each heute($liste) as aufgabe}
			<AufgabeBox aufgabe={aufgabe}/>
		{/each}
	</div>
	<div class="liste">
		<header>{ $i18n.t('abbild.gerade.header-unabhängig') }</header>
		<div use:dndzone={{items, flipDurationMs}} on:consider={handleDndConsider} on:finalize={handleDndFinalize}>
		{#each items as aufgabe (aufgabe.id)}
			<div animate:flip={{duration: flipDurationMs}}>
				<AufgabeBox aufgabe={aufgabe}/>
			</div>
		{/each}
		</div>
	</div>
	<div class="liste" class:liste-verbergen={restVerbergen}>
		<header class="komplex">
			<div class="titel">{ $i18n.t('abbild.gerade.header-rest') }</div>
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
			<AufgabeBox aufgabe={aufgabe}/>
		{/each}
	</div>
{:else}
	<section class="message">
		<p>{@html $i18n.t('abbild.gerade.leer-empfehlung') }</p>
	</section>
{/if}

<style lang="scss">
	@import './liste.scss';
	.buch {
		display: flex;
		flex-direction: row-reverse;
		> .liste {
			padding: .5rem 0;
			flex: 1;
		}
	}
	:global(code) {
		font-family: serif;
		color: #000;
		background-color: rgba(0,0,0, .1);
	}
</style>
