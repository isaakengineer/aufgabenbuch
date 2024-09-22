<script>
    import { Aufgabe, updateStore } from './store.js';
    import { liste, addAufgabe } from '../liste/store.js';
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/core";

    import { format } from "date-fns";

    import ResetAufgabe from './ResetAufgabe.svelte';
    import Erledigen from './Erledigen.svelte';
    import Notiz from './Notiz.svelte';

    import { Aussehen, Darstellung } from '../routes/store.js';
    import { Value } from 'sass';

    let formData = {};
    let wochentagOptions = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag'];
    let prioritaetOptions = [0, 1, 2, 3, 4];
    let neueAufgabe = {};
    let neueAufgabeId = null;
    let rueckmeldung = false;

    onMount(() => {
        Aufgabe.subscribe(value => {
            formData = value;
        });
    });

    function handleOption(event) {
        const { name, value } = event.target;
        let intValue = parseInt(value, 10);
        updateStore[{ [name]: intValue }]
    }
    function handleChange(event) {
        
        const { name, value } = event.target;
        console.log(`changing ${name} to ${value}`);
        console.log(typeof(value))
        formData[name] = value;
        updateStore({ [name]: value });
        console.log($Aufgabe)
        console.log(formData)
    }

    let fokus = 'normal';
    if (formData.id === undefined || formData.id === null) {
        fokus = 'normal';
    } else {
        fokus = 'aktionen';
    }
    const resetFormular = () => {
        Aufgabe.reset();
        rueckmeldung = false;
    }

    async function hinfuegen() {
        if ( formData.beschreibung.length == 0 ) {
            rueckmeldung = "Beschreibung darf nicht leer sein!";
        } else {
            console.log("hinfuegen", formData);
            neueAufgabe = await invoke('aufgabe_hinfuegen', { aufgabe: formData });
            addAufgabe(neueAufgabe);
            resetFormular();
        }
    }
    const aendern = async function() {
        if ( formData.beschreibung.length == 0 ) {
            rueckmeldung = "Beschreibung darf nicht leer sein!";
        } else {
            let aufgabe = $Aufgabe;
            neueAufgabe = await invoke('aufgabe_aendern', { aufgabe: aufgabe });
            rueckmeldung = false;
        }
        // TODO: ersetze die alte Aufgabe mit der neuen Aufgabe
    }

    function setFokus(tab) {
        fokus = tab;
    }

    const datumLeserlich = (datumString) => {
		return format( new Date(datumString), "yyyy-MM-dd");
	}
</script>

<form>
    {#if rueckmeldung}<aside>
        <p>{ rueckmeldung }</p>
    </aside>{/if}
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
                    <input type="date" name="erstellt_an" value={formData.erstellt_an ? datumLeserlich(formData.erstellt_an) : ''} disabled />
                </label>

                <label>
                    Geändert am:
                    <input type="date" name="geaendert_an" value={formData.geaendert_an ? datumLeserlich(formData.geaendert_an) : ''} disabled />
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
                        <select name="wochentag" on:change={handleOption} bind:value={formData.wochentag} placeholder="Wochentag">
                            {#each $Aussehen.optionen.wochentagen as wochentag}
                                <option value={wochentag.id}>{wochentag.name}</option>
                            {/each}
                        </select>
                    </div>
                    <div>
                        <label for="prioritaet">Prioritaet</label>
                        <select name="prioritaet" on:change={handleOption} bind:value={formData.prioritaet} placeholder="Priorität">
                            {#each $Aussehen.optionen.prioritaeten as prioritaet}
                                <option value={prioritaet.id}>{prioritaet.name}</option>
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
        <button on:click={resetFormular}>Reset</button>
        <div class="tabs">
            <div class={`tab ${fokus === 'normal' ? 'active' : ''}`} on:click={() => setFokus('normal')}>Wesentliche</div>
            <div class={`tab ${fokus === 'link' ? 'active' : ''}`} on:click={() => setFokus('link')}>Link</div>
            <div class={`tab ${fokus === 'notiz' ? 'active' : ''}`} on:click={() => setFokus('notiz')}>Notiz</div>
            <div class={`tab ${fokus === 'aktionen' ? 'active' : ''}`} on:click={() => setFokus('aktionen')}>Aktionen</div>
        </div>
        {#if $Aufgabe.id}
            <button on:click={aendern}>Ändern</button>
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
form > aside {
    padding: 0px 0;
    > p {
        margin: .2rem;
        padding: .2rem 1rem;
    }
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