<script>
	import { liste } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";
	import { Aussehen, Ausstattung } from "../routes/store.js";
	import { CircleDashed } from "phosphor-svelte";

	let aufgabe = {};
	let prioritäten = $Aussehen.optionen.prioritaeten;

	const priorisieren = async (aufgabe, prioritaet) => {
		console.log(aufgabe, prioritaet);
	}
</script>

<div class="prioritaet-auswahl">
	{#if prioritäten.length > 0}
		{#each prioritäten as p}
			<button class="p" on:click={() => priorisieren(aufgabe, p)}>
				<CircleDashed />
			</button>
		{/each}
	{/if}
</div>

<style lang='scss'>
.prioritaet-auswahl {
	display: flex;
	gap: 1px;
}
</style>
