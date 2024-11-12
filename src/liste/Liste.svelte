<script>
	import i18n from '$lib/i18n';
	import { liste, gruppen } from "./store.js";
	import { Ausstattung } from "../routes/store.js";

	import Empty from "phosphor-svelte/lib/Empty";

	import AufgabeBox from './AufgabeBox.svelte';

	let items =[];

	liste.subscribe((value) => {
		items = value;
	});

	let aufgaben = $liste;
	let filtern = false;
	let gruppe = false;

	const filterNachGruppe = (liste, g) => {
		console.log(liste)
		return liste.filter((a) => {
			if (gruppe)	return (a.gruppe === g)
			else return (a.gruppe === null)
		})
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
				{#each filterNachGruppe($liste, gruppe) as aufgabe (aufgabe.id)}
					<AufgabeBox aufgabe={aufgabe}/>
				{/each}
			</div>
		{:else}
			<div class="liste">
				{#each items as aufgabe (aufgabe.id)}
					<AufgabeBox aufgabe={aufgabe}/>
				{/each}
			</div>
		{/if}
	</div>
{:else}
	<section class="message">
		<p>{ $i18n.t('abbild.liste.leer') }</p>
		<p>{ $i18n.t('abbild.liste.leer-empfehlung') }</p>
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
