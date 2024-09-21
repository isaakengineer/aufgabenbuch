<script>
    import { invoke } from "@tauri-apps/api/core";
    let file;

    async function file_waehlen() {
        file = await invoke("file_waehlen");  
    }
    const fileErstellen = async () => {
        file = await invoke("file_erstellen");
        console.log(file);
    }

</script>

<section class="message">
    <h1>Wilkommen auf Aufgabenbuch</h1>
    <p>Noch kein Datenbank für ihre Aufgaben ist vorhanden.</p>
    <p>Um die Lage zu verbessern, bitte entweder wählen sie eine breits existierende Datenbank oder ein Ort ein neues Datenbank zu erstellen.</p>
    <button on:click={file_waehlen}>File wählen</button>
    <button on:click={fileErstellen}>File schaffen</button>
</section>

<style lang="scss">
</style>