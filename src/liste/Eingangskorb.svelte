<script>
	import { invoke } from "@tauri-apps/api/core";

	import i18n from '$lib/i18n';

	import { liste, aufgabeAendern } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";
	import { Aussehen, Ausstattung } from "../routes/store.js";
	import Priorisieren from "./Priorisieren.svelte";

	import Circle from 'phosphor-svelte/lib/Circle';
	import CircleDashed from 'phosphor-svelte/lib/CircleDashed';
	import CaretLineRight from 'phosphor-svelte/lib/CaretLineRight';
	import CaretLineLeft from 'phosphor-svelte/lib/CaretLineLeft';


	let prioritäten = $Aussehen.optionen.prioritaeten;
	const priorisieren = async (aufgabe, prioritaet) => {
		console.log(aufgabe, prioritaet);
		let neueAufgabe = await invoke('aufgabe_priorisieren', {
			id: aufgabe.id, prioritaet: prioritaet.id
		})
		aufgabeAendern(aufgabe.id, neueAufgabe)
	}
	const gezeigtePrioritaetAndern = async (nummer) => {
		$Ausstattung.gezeigtePrioritaet = nummer;
		$liste = await invoke("prioritaetenliste", { prioritaet: $Ausstattung.gezeigtePrioritaet });
	}
	let gezeigtePrioritaetName = (pr) => {
		let p = $Aussehen.optionen.prioritaeten.find((p) => p.id === pr);
		return $i18n.t('priorität-optionen.' + p.id);
	}
</script>

<div class="abbild">
	<header>
		{#if $Ausstattung.gezeigtePrioritaet > 0 }
			<button on:click={() => gezeigtePrioritaetAndern($Ausstattung.gezeigtePrioritaet -1)}>
				<CaretLineLeft />
			</button>
		{/if}
		<div class="titel">
			{ gezeigtePrioritaetName($Ausstattung.gezeigtePrioritaet)	}
		</div>
		{#if $Ausstattung.gezeigtePrioritaet < 5 }
			<button on:click={() => gezeigtePrioritaetAndern($Ausstattung.gezeigtePrioritaet +1)}>
				<CaretLineRight />
			</button>
		{/if}
	</header>
	<div class="aufgabentabelle">
		{#if $liste.length > 0}
			<div class="tabelle-kopf">
				<div>{ $i18n.t('aufgabe.id') }</div>
				<div>{ $i18n.t('aufgabe.beschreibung') }</div>
				{#if prioritäten.length > 0}
					{#each prioritäten as p}
						<div class="prioritaet-auswahl">{ $i18n.t('priorität-optionen.' + p.id) }</div>
					{/each}
				{/if}
			</div>

			<div class="tabelle-daten">
				{#each $liste as aufgabe}
					<div
						class="aufgabe"
						class:gewaehlt={ ($Aufgabe.id === aufgabe.id)}
						on:click={() => $Aufgabe = aufgabe}
					>
						<div class="id">{ aufgabe.id }</div>
						<div class="satz">{ aufgabe.beschreibung }</div>
						{#if prioritäten.length > 0}
							{#each prioritäten as p}
								<div class="prioritaet-auswahl">
									{#if aufgabe.prioritaet !== p.id }
										<button class="p" on:click={() => priorisieren(aufgabe, p)}>
											<CircleDashed />
										</button>
									{:else}
										<button class="p" on:click={() => priorisieren(aufgabe, p)}>
											<Circle weight="fill" />
										</button>
									{/if}
								</div>
							{/each}
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style lang='scss'>
.abbild {
	max-height: 100%;
	overflow: hidden;
	display: flex;
	flex-direction: column;
}
header {
	padding: 1rem;
	display: flex;
	justify-content: space-between;
	.titel {
		font-size: 1.2rem;
		text-transform: capitalize;
		font-family: "LatoWebBold";
	}
}
.aufgabentabelle {
	display: flex;
	flex-direction: column;
	grid-template-rows: 1fr 1px 1fr;
	overflow: hidden;
	max-height: 100%;
	> .tabelle-kopf {
		margin-bottom: 2px;
	}
	> .tabelle-kopf, .tabelle-daten > div {
		display: grid;
		grid-template-columns: 3rem 1fr repeat(6, 3rem);
		gap: 1px
	}
	> .tabelle-kopf > div , .tabelle-daten > div > div {
		background-color: #fff;
	}
	> .tabelle-kopf .prioritaet-auswahl {
		padding: .5rem .2rem;
  		writing-mode: vertical-lr;
   		text-transform: capitalize;
	}
	> .tabelle-kopf > div {
		text-align: center;
		align-content: center;
	}
	> .tabelle-daten .prioritaet-auswahl {
		text-align: center;
		align-content: center;
	}
	> .tabelle-daten {
		display: flex;
		flex-direction: column;
		gap: 1px;
		overflow-y: auto;
		> .aufgabe {
			.satz, .id {
				padding: .2rem .4rem;
			}
			> div {
			}
		}
	}
}
</style>
