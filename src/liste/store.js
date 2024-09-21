import { writable } from 'svelte/store';

const liste = writable([]);

function addAufgabe(newAufgabe) {
    liste.update(currentList => [...currentList, newAufgabe]);
}

export { liste, addAufgabe };