import { writable, derived } from 'svelte/store';

function createAufgabe() {
    const { subscribe, set, update } = writable({
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

    return {
        subscribe,
        set,
        update,
        reset: () => set({
            id: null,
            gruppe: '',
            beschreibung: '',
            notiz: '',
            link: '',
            wochentag: 0,
            prioritaet: 0,
            position: null,
            verschoben: null,
            getan: null,
            vernachlaessigt: null,
            kommentar: '',
            erstellt_an: null,
            geaendert_an: null
        }),
        markAsDone: () => update(n => ({ ...n, getan: new Date() }))
    };
}

export const Aufgabe = createAufgabe();

export const AufgabeIstErledigt = derived(Aufgabe, $Aufgabe => {
    if ($Aufgabe.vernachlaessigt !== null) {
        return 'vernachlaessigt';
    } else if ($Aufgabe.getan !== null) {
        return 'getan';
    } else if ($Aufgabe.verschoben !== null) {
        return 'verschoben';
    }
});
AufgabeIstErledigt.set = (value) => {
    Aufgabe.update(current => ({ ...current, verschoben: null, getan: null, vernachlaessigt: null }));
    if (value === 'vernachlaessigt') {
        Aufgabe.update(current => ({ ...current, vernachlaessigt: new Date().toISOString() }));
    } else if (value === 'getan') {
        Aufgabe.update(current => ({ ...current, getan: new Date().toISOString() }));
    } else if (value === 'verschoben') {
        Aufgabe.update(current => ({ ...current, verschoben: new Date().toISOString() }));
    }
}

// Example function to initialize the store or update fields
export function updateStore(data) {
    Aufgabe.update(current => ({
        ...current,
        ...data
    }));
}