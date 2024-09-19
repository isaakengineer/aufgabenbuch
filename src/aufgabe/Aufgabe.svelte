<script>
    import { Aufgabe, updateStore } from './store.js';
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/core";

    import ResetAufgabe from './ResetAufgabe.svelte';
    import Erledigen from './Erledigen.svelte';
    import Notiz from './Notiz.svelte';


  
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

    let fokus = 'normal';
    if (formData.id === undefined || formData.id === null) {
        fokus = 'normal';
    } else {
        fokus = 'aktionen';
    }

    async function hinfuegen() {
        console.log("hinfuegen", formData);
        await invoke('aufgabe_hinfuegen', { beschreibung: formData.beschreibung });
    }
</script>

<form>
    {#if import.meta.env.DEV}
        <fieldset id="dev">
            <div>
                <input type="text" name="gruppe" value={formData.gruppe} disabled placeholder="Gruppe" />
            </div>
            <div>
                <input type="number" name="id" value={formData.id} disabled placeholder="id" size="5"/>
            </div>
            <div>
                <input type="number" name="position" value={formData.position} disabled placeholder="Position" />
            </div>
            <label>
                Erstellt am:
                <input type="date" name="erstellt_an" value={formData.erstellt_an ? formData.erstellt_an.toISOString().split('T')[0] : ''} disabled />
            </label>

            <label>
                Geändert am:
                <input type="date" name="geaendert_an" value={formData.geaendert_an ? formData.geaendert_an.toISOString().split('T')[0] : ''} disabled />
            </label>
        </fieldset>
    {/if}
    {#if fokus != 'notiz'}
        <fieldset id="satz">
            <textarea name="beschreibung" on:input={handleChange} bind:value={formData.beschreibung} placeholder="Beschreibung"></textarea>
        </fieldset>
        <fieldset id="actions">
            <legend>Aktionen</legend>
            <button on:click={hinfuegen}>Neu</button>
            <div class="btn-group">
                <button on:click={() => fokus = 'normal'}>Normal</button>
                <button on:click={() => fokus = 'notiz'}>Notiz</button>
                <button on:click={() => fokus = 'link'}>Link</button>
                <button on:click={() => fokus = 'aktionen'}>Aktionen</button>
            </div>
            <ResetAufgabe />
        </fieldset>
        <fieldset>
            {#if fokus === 'normal'}
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
                    (?Notiz)
                </div>
                <div>
                    (?Link)
                </div>
            {:else if fokus === 'link'}
                <input type="text" name="link" on:input={handleChange} bind:value={formData.link} placeholder="Link" />
            {:else if fokus === 'aktionen'}
                <Erledigen />
            {/if}
        </fieldset>
    {:else}
        <fieldset id="notiz">
            <legend>Notiz</legend>
            <Notiz />
            <button class="close" on:click={() => fokus = 'normal'}>X</button>
        </fieldset>
    {/if}
</form>

<style lang="scss">
:global(textarea) {
    border-radius: 0;
    border: 0;
}
form {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: .2rem;
    padding: .2rem 0;
}
fieldset {
    margin: 0;
    padding: 0 .2rem;
    display: flex;
    border: none;
    textarea {
        flex: 1;
        padding: .5rem;
    }
    legend {
        font-size: .8rem;
        text-transform: lowercase;
    }
}
#dev {
    background-color: lightgray;
    flex-wrap: wrap;
}
button.close {
    position: absolute;
    bottom: 0;
    right: 0;
}
fieldset#notiz {
    padding: 0;
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
    > :global(textarea) {
        flex: 1;
        height: 100%;
        padding: .5rem;
    }
}
#actions {
    display: flex;
    justify-content: space-between;
}
.btn-group {
    > button {
        margin: 0;
        border-radius: 0px;
        float: right;
        border: 1px solid black;
    }
}
</style>