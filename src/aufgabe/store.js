import { writable } from 'svelte/store';

export const Aufgabe = writable({
    id: null,                   // integer (disabled)
    gruppe: '',                 // string (disabled)
    beschreibung: '',           // string (textarea)
    notiz: '',                  // text (textarea)
    link: '',                   // text (input)
    wochentag: 0,               // integer (drop down 0-7)
    prioritaet: 0,              // integer (drop down 0-4)
    position: null,             // integer (disabled)
    
    verschoben: null,           // date (checkbox)
    getan: null,                // date (checkbox)
    vernachlaessigt: null,      // date (checkbox)
    kommentar: '',              // string (input)

    erstellt_an: null,          // date (disabled)
    geaendert_an: null          // date (disabled)
});

// Example function to initialize the store or update fields
export function updateStore(data) {
    Aufgabe.update(current => ({
        ...current,
        ...data
    }));
}