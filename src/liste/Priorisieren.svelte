<script>
	import { liste } from "./store.js";
	import { Aufgabe } from "../aufgabe/store.js";
	import { Aussehen, Ausstattung } from "../routes/store.js";


	let aufgabe = {};


	const priorisieren = async (aufgabe, prioritaet) => {
		console.log(aufgabe, prioritaet);
	}
</script>





<style lang='scss'>
.prioritaet-auswahl {
	display: flex;
	gap: 1px;
}
</style>
