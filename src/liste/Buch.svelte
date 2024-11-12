<script>
	import { compareDesc } from "date-fns";

	import i18n from '$lib/i18n';
	import { liste, aufgabeErlediegungsStatus } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";
	import AufgabeBox from './AufgabeBox.svelte';

	let aufgaben = $liste;

	const nachDatumSortieren = (liste) => {
		let neueListe = liste.sort((a,b) => {
			let a_datum = aufgabeErlediegungsStatus(a).date;
			let b_datum = aufgabeErlediegungsStatus(b).date;
			return compareDesc(a_datum, b_datum );
		})
		return neueListe;
	}
</script>

<div class="buch">
	{#if $liste.length > 0}
		<div class="liste">
			{#each nachDatumSortieren($liste) as aufgabe}
				<AufgabeBox aufgabe={aufgabe}/>
			{/each}
		</div>
	{:else}
		<section class="message">
			<p>{ $i18n.t('abbild.buch.leer') }</p>
			<p>{ $i18n.t('abbild.buch.leer-empfehlung') }</p>
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
