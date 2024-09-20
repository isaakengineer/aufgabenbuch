<script>
    import { liste } from './store.js';
    import { Aufgabe } from '../aufgabe/store.js'
    
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
    async function aufgabeGewaelt(aufgabe) {
        $Aufgabe = aufgabe;
    }

    const istErledigt = (a) => {
        return a.getan !== null || a.vernachlaessigt !== null || a.verschoben !== null;
    };

  </script>

{#if $liste.length > 0 }
  <div class="liste">
    <header>Aufgaben liste</header>
    {#each $liste as aufgabe}
      <div class="aufgabe" class:erledigt={istErledigt(aufgabe)} on:click={() => aufgabeGewaelt(aufgabe)}>
          <div class="id">{aufgabe.id}</div>
          <div class="beschreibung">{aufgabe.beschreibung}</div>
          {#if istErledigt(aufgabe)}
            <div class="kommentar">{aufgabe.kommentar}</div>
          {/if}
          <!-- {#if import.meta.env.DEV}
            <div class="dev debug">
              <div>Gruppe: {aufgabe.gruppe}</div>
              <div>Notiz: {aufgabe.notiz}</div>
              <div>Link: <a href={aufgabe.link} target="_blank">{aufgabe.link}</a></div>
              <div>Wochentag: {aufgabe.wochentag}</div>
              <div>Priorität: {aufgabe.prioritaet}</div>
              <div>Position: {aufgabe.position !== null ? aufgabe.position : 'N/A'}</div>
              <div>Verschoben: {aufgabe.verschoben ? aufgabe.verschoben : 'N/A'}</div>
              <div>Getan: {aufgabe.getan ? aufgabe.getan : 'N/A'}</div>
              <div>Vernachlässigt: {aufgabe.vernachlaessigt ? aufgabe.vernachlaessigt : 'N/A'}</div>
              <div>Kommentar: {aufgabe.kommentar}</div>
              <div>Erstellt am: {aufgabe.erstellt_an ? aufgabe.erstellt_an : 'N/A'}</div>
              <div>Geändert am: {aufgabe.geaendert_an ? aufgabe.geaendert_an : 'N/A'}</div>
            </div>
          {/if} -->
      </div>
    {/each}
  </div>
{:else}
  <section class="message">
    <p>Gerade gibt es keine Aufgaben auf Ihre Liste.</p>
    <p>Vielleicht versuchen Sie ein Paar Aufgaben auf Ihre Liste hinzufügen, indem Sie die daunten gedrückte Formulare nutzen.</p>
  </section>
{/if}

<style lang="scss">
.liste {
  display: flex;
  flex-direction: column;
  gap: .2rem;
}
.aufgabe {
  > div {
    display: inline;
    padding: .2rem;
  }
  > .id {
    background-color: #eee;
    
    border-radius: .2rem;
    margin-right: .2rem;
  }
  > .kommentar {
    color: rgb(6, 6, 100);
  }
  margin: 0 .2rem;
    // display: flex;
    // flex-wrap: wrap;
    // gap: .5rem;
    > .beschreibung {
      flex: 1;
    }
    &.erledigt {
      > .beschreibung {
        text-decoration: line-through;
      }
    }
    > .dev {
      flex: 1;
      display: flex;
      font-size: .9em;
      flex-wrap: wrap;
    }
    box-shadow: 0px 0px 1px black;
    // padding: .4rem;
    background-color: #ddd;
    &:hover {
      background-color: #eee;
      cursor: pointer;
    }
}
</style>