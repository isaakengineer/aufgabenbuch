<script>
    import { Aufgabe, updateStore } from './store.js';
</script>

<!-- <label>
            Verschoben:
            <input type="checkbox" name="verschoben"  bind:checked={formData.verschoben} />
        </label>

        <label>
            Getan:
            <input type="checkbox" name="getan"  bind:checked={formData.getan} />
        </label>

        <label>
            Vernachlässigt:
            <input type="checkbox" name="vernachlaessigt"  bind:checked={formData.vernachlaessigt} />
        </label> -->

<input type="text" name="kommentar" bind:value={$Aufgabe.kommentar} placeholder="Kommentar" />