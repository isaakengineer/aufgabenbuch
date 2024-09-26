import { writable } from "svelte/store";

const liste = writable([]);
const gruppen = writable([]);

function addAufgabe(newAufgabe) {
	liste.update((currentList) => [...currentList, newAufgabe]);
}

export { liste, addAufgabe, gruppen };
