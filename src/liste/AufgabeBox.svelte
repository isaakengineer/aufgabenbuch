<script>
	import { format, compareDesc } from "date-fns";
	import i18n from '$lib/i18n';
	import { Aufgabe } from "../aufgabe/store.js";

	import Circle from 'phosphor-svelte/lib/Circle';
	import { aufgabeErlediegungsStatus } from './store.js';

	export let aufgabe;

	const formatDate = (date) => {
		return format(new Date(date), "yyyy-MM-dd"); // Format the date as 'day.month.year'
	};

	const istErledigt = (a) => {
		return (
			a.getan !== null ||
			a.vernachlaessigt !== null ||
			a.verschoben !== null
		);
	};
</script>

<div
	class="aufgabe"
	class:erledigt={ istErledigt(aufgabe) }
	class:gewaehlt={ ($Aufgabe.id === aufgabe.id) }
	on:click={() => $Aufgabe = aufgabe }
>
	<div class="satz">
		<div class="id">{aufgabe.id}</div>
		{#if istErledigt(aufgabe)}
			<div>
				<div class="erledigt">
					<div class="datum">{formatDate(aufgabeErlediegungsStatus(aufgabe).date)}</div>
					<div class="wand"> </div>
					<div class="status">{  $i18n.t('aufgabe.erledigt-optionen.' + aufgabeErlediegungsStatus(aufgabe).status) }</div>
				</div>
			</div>
			{#if aufgabe.kommentar}
				<div class="kommentar">{aufgabe.kommentar}</div>
			{/if}
		{/if}
		{#if aufgabe.notiz || aufgabe.link}
			<div class="extra">
				{#if aufgabe.notiz}<div class="notiz"><Circle weight="fill" size=".8em" /></div>{/if}
				{#if aufgabe.link}<div class="link"><Circle weight="fill" size=".8em"/></div>{/if}
			</div>
		{/if}
		<div class="beschreibung">
			{aufgabe.beschreibung}
		</div>
	</div>
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

<style lang="scss">
	@import './aufgabe.scss';
</style>
