import { writable, derived, readable } from "svelte/store";

const wochentagen = [
	{ id: 0, name: "keine" },
	{ id: 1, name: "Montag" },
	{ id: 2, name: "Dienstag" },
	{ id: 3, name: "Mittwoch" },
	{ id: 4, name: "Donnerstag" },
	{ id: 5, name: "Freitag" },
	{ id: 6, name: "Samstag" },
	{ id: 7, name: "Sonntag" },
];

const prioritaeten = [
	{ id: 0, name: "eingangskorb" },
	{ id: 5, name: "jetzt" },
	{ id: 4, name: "nächste" },
	{ id: 3, name: "später" },
	{ id: 2, name: "irgendwann" },
	{ id: 1, name: "bnous" },
];

const Ausstattung = writable({
	identitaet: {
		dateipfad: "",
		name: "",
		endung: "",
	},
	haupt: "nichts",
	gruppenZeigen: false,
	gezeigtePrioritaet: 0,
});
const Aussehen = readable({
	optionen: {
		wochentagen: wochentagen,
		prioritaeten: prioritaeten,
	},
});

export { Ausstattung, Aussehen };
