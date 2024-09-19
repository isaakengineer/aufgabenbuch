<script>
    import { Aufgabe, updateStore } from './store.js';
    import { onMount } from 'svelte';

    // import ResetAufgabe from './src/aufgabe/ResetAufgabe.svelte';
    import Erledigen from './Erledigen.svelte';

    let formData = {};
    let wochentagOptions = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'];
    let prioritaetOptions = [0, 1, 2, 3, 4];

    onMount(() => {
        Aufgabe.subscribe(value => {
            formData = value;
        });
    });

    function handleChange(event) {
        const { name, value } = event.target;
        updateStore({ [name]: value });
    }
</script>

<form>
    <fieldset>
        <div>
            <input type="text" name="gruppe" value={formData.gruppe} disabled placeholder="Gruppe" />
        </div>

        <div>
            <textarea name="beschreibung" on:input={handleChange} bind:value={formData.beschreibung} placeholder="Beschreibung"></textarea>
        </div>
    </fieldset>
    <fieldset>
        <div>
            <textarea name="notiz" on:input={handleChange} bind:value={formData.notiz} placeholder="Notiz"></textarea>
        </div>

        <div>
            <input type="text" name="link" on:input={handleChange} bind:value={formData.link} placeholder="Link" />
        </div>
    </fieldset>
    <fieldset>
        <div>
            <select name="wochentag" on:change={handleChange} bind:value={formData.wochentag} placeholder="Wochentag">
                <option value="" disabled selected>Wochentag</option>
                {#each wochentagOptions as wochentag}
                    <option value={wochentag}>{wochentag}</option>
                {/each}
            </select>
        </div>

        <div>
            <select name="prioritaet" on:change={handleChange} bind:value={formData.prioritaet} placeholder="Priorität">
                <option value="" disabled selected>Priorität</option>
                {#each prioritaetOptions as prioritaet}
                    <option value={prioritaet}>{prioritaet}</option>
                {/each}
            </select>
        </div>

        <div>
            <input type="number" name="position" value={formData.position} disabled placeholder="Position" />
        </div>

        <!-- <label>
            Verschoben:
            <input type="checkbox" name="verschoben" on:change={handleChange} bind:checked={formData.verschoben} />
        </label>

        <label>
            Getan:
            <input type="checkbox" name="getan" on:change={handleChange} bind:checked={formData.getan} />
        </label>

        <label>
            Vernachlässigt:
            <input type="checkbox" name="vernachlaessigt" on:change={handleChange} bind:checked={formData.vernachlaessigt} />
        </label>

        <label>
            Kommentar:
            <input type="text" name="kommentar" on:input={handleChange} bind:value={formData.kommentar} />
        </label> -->

        <!-- <label>
            Erstellt am:
            <input type="date" name="erstellt_an" value={formData.erstellt_an ? formData.erstellt_an.toISOString().split('T')[0] : ''} disabled />
        </label>

        <label>
            Geändert am:
            <input type="date" name="geaendert_an" value={formData.geaendert_an ? formData.geaendert_an.toISOString().split('T')[0] : ''} disabled />
        </label> -->
    </fieldset>
</form>