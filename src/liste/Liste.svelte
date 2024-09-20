<script>
    import { liste } from './store.js';
  
    let aufgaben = $liste;
  
    // Add a new Aufgabe to the liste
    function addAufgabe(aufgabe) {
      liste.update(n => [...n, aufgabe]);
    }
  
    // Remove an Aufgabe from the liste by its index
    function removeAufgabe(index) {
      liste.update(n => n.filter((_, i) => i !== index));
    }
  
    // Update an existing Aufgabe in the liste
    function updateAufgabe(index, aufgabe) {
      liste.update(n => n.map((item, i) => (i === index ? aufgabe : item)));
    }
  </script>

{#each $liste as aufgabe}
    <header>Aufgaben liste</header>
    <div class="aufgabe">
        <h3>Gruppe: {aufgabe.gruppe}</h3>
        <p>Beschreibung: {aufgabe.beschreibung}</p>
        <p>Notiz: {aufgabe.notiz}</p>
        <p>Link: <a href={aufgabe.link} target="_blank">{aufgabe.link}</a></p>
        <p>Wochentag: {aufgabe.wochentag}</p>
        <p>Priorität: {aufgabe.prioritaet}</p>
        <p>Position: {aufgabe.position !== null ? aufgabe.position : 'N/A'}</p>
        <p>Verschoben: {aufgabe.verschoben ? aufgabe.verschoben : 'N/A'}</p>
        <p>Getan: {aufgabe.getan ? aufgabe.getan : 'N/A'}</p>
        <p>Vernachlässigt: {aufgabe.vernachlaessigt ? aufgabe.vernachlaessigt : 'N/A'}</p>
        <p>Kommentar: {aufgabe.kommentar}</p>
        <p>Erstellt am: {aufgabe.erstellt_an ? aufgabe.erstellt_an : 'N/A'}</p>
        <p>Geändert am: {aufgabe.geaendert_an ? aufgabe.geaendert_an : 'N/A'}</p>
    </div>
{/each}

<style lang="scss">
.aufgabe {
    display: flex;
}
</style>