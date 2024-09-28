import { writable } from "svelte/store";

const liste = writable([]);
const gruppen = writable([]);

function addAufgabe(newAufgabe) {
	liste.update((currentList) => [...currentList, newAufgabe]);
}

function aufgabeAendern(index, aufgabe) {
	liste.update((n) => n.map((item) => (item.id === index ? aufgabe : item)),);
}

export { liste, addAufgabe, gruppen, aufgabeAendern };
