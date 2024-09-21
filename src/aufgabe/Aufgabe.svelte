<script>
    import { Aufgabe, updateStore } from './store.js';
    import { liste, addAufgabe } from '../liste/store.js';
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/core";

    import ResetAufgabe from './ResetAufgabe.svelte';
    import Erledigen from './Erledigen.svelte';
    import Notiz from './Notiz.svelte';


  
    let formData = {};
    let wochentagOptions = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'];
    let prioritaetOptions = [0, 1, 2, 3, 4];
    let neueAufgabe = {};
    let neueAufgabeId = null;

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
        neueAufgabe = await invoke('aufgabe_hinfuegen', { beschreibung: formData.beschreibung });
        addAufgabe(neueAufgabe);
        Aufgabe.reset();
    }

    function setFokus(tab) {
        fokus = tab;
    }
</script>

<form>
    <div class="content">
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
            
            <fieldset id="extra">
                {#if fokus === 'normal'}
                    <div>
                        <label for="wochentag">Wochentag</label>
                        <select name="wochentag" on:change={handleChange} bind:value={formData.wochentag} placeholder="Wochentag">
                            <option value="" disabled selected>Wochentag</option>
                            {#each wochentagOptions as wochentag}
                                <option value={wochentag}>{wochentag}</option>
                            {/each}
                        </select>
                    </div>
                    <div>
                        <label for="prioritaet">Prioritaet</label>
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
                <Notiz />
                <button class="close" on:click={() => fokus = 'normal'}>X</button>
            </fieldset>
        {/if}
    </div>
    <div class="tabs-container">
        <ResetAufgabe />
        <div class="tabs">
            <div class={`tab ${fokus === 'normal' ? 'active' : ''}`} on:click={() => setFokus('normal')}>Wesentliche</div>
            <div class={`tab ${fokus === 'aktionen' ? 'active' : ''}`} on:click={() => setFokus('aktionen')}>Aktionen</div>
            <div class={`tab ${fokus === 'link' ? 'active' : ''}`} on:click={() => setFokus('link')}>Link</div>
            <div class={`tab ${fokus === 'notiz' ? 'active' : ''}`} on:click={() => setFokus('notiz')}>Notiz</div>
        </div>
        {#if $Aufgabe.id}
            <button on:click={() => console.log('fun schreiben')}>Ändern</button>
        {:else}
            <button on:click={hinfuegen}>Neu</button>
        {/if}
    </div>
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
    padding: 0;
    background-color: #fff;
    border: 0;
}
fieldset {
    margin: 0;
    padding: .4rem;
    display: flex;
    border: transparent;
    border-radius: 0;
    :global(input) {
        flex: 1;
        padding: .2rem;
    }
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
    gap: .2rem;
    justify-content: space-around;
    font-size: .9rem;
    padding: .2rem;
    margin: .2rem;
    border: 1px solid #777;
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
        // height: 100%;
        padding: .5rem;
    }
}
fieldset#extra {
    display: flex;
    justify-content: space-between;
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

.tabs-container {
    display: flex;
    background-color: #fff;
    justify-content: space-between;
    :global(> button) {
        margin: .3rem .4rem;
    }
}
.tabs {
    display: flex;
    cursor: pointer;
    margin-bottom: .1rem;
    gap: .1rem;
}
.tab {
    font-size: .9rem;
    padding: .15rem .5rem 0 .5rem;
    margin-bottom: .2rem;
    border: 1px solid #ccc;
    border-top: none;
    background: lightgray;
    border-radius: 0 0 .2rem .2rem;
}

.tab.active {
    background: #eee;
    font-weight: bold;
    border-color: #ccc;
}

.content {
    border: 1px solid #ccc;
    border-radius: 5px;
    padding: .2rem;
    box-sizing: border-box;
    margin: .4rem;
    margin-bottom: 0;
    background: #eee;
}
</style>