import { writable } from "svelte/store";

const liste = writable([]);
const gruppen = writable([]);

function addAufgabe(newAufgabe) {
	liste.update((currentList) => [...currentList, newAufgabe]);
}

function removeAufgabe(index) {
	liste.update((n) => n.filter((_, i) => i !== index));
}

function aufgabeAendern(index, aufgabe) {
	liste.update((n) => n.map((item) => (item.id === index ? aufgabe : item)),);
}

const aufgabeErlediegungsStatus = (a) => {
	let readable = {
		date: null,
		status: "No status available",
	};
	if (a.getan !== null) {
		readable.status = "getan";
		readable.date = a.getan;
	} else if (a.vernachlaessigt !== null) {
		readable.status = "vernachlaessigt";
		readable.date = a.vernachlaessigt;
	} else if (a.verschoben !== null) {
		readable.status = "verschoben";
		readable.date = a.verschoben;
	}
	return readable;
};

export { liste, addAufgabe, aufgabeAendern, gruppen, aufgabeErlediegungsStatus };
