<script>
    import { event } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    let file;

    import { listen } from '@tauri-apps/api/event'

    listen('tauri://drag-enter', async (event) => {
        console.log("fire event", event)
        file = await invoke("dateipfad_eingegeben", {
            pfad: event.payload.paths[0]
        })
    })
    // listen('tauri://drag-over', event => {
    //     console.log(event)
    // })
    // listen('tauri://drag-leave', event => {
    //     console.log(event)
    // })
    // listen("tauri://drag-drop", async (event) => {
    //     console.log(event)
    //     file = await invoke("dateipfad_eingegeben", {
    //         pfad: event.payload.paths[0]
    //     })
    // })

    async function file_waehlen() {
        file = await invoke("file_waehlen");
    }
    const fileErstellen = async () => {
        file = await invoke("file_erstellen");
        console.log(file);
    }

    const dropped = async (event) => {
        // event.preventDefault();
        const files = event.dataTransfer.files;
        if (files.length > 0) {
            const filePath = files[0].path;
            console.log("Dropped file path:", filePath);
            // You can now use the file path as needed
        }
    }
    const allowDrop = (event) => {
        // event.preventDefault();
    }

</script>

<div class="wilkomen-seite">
    <section class="message">
        <h1>Wilkommen auf Aufgabenbuch</h1>
        <p>Noch kein Datenbank für ihre Aufgaben ist vorhanden.</p>
        <p>Um die Lage zu verbessern, bitte entweder wählen sie eine breits existierende Datenbank oder ein Ort ein neues Datenbank zu erstellen.</p>
        <button on:click={file_waehlen}>File wählen</button>
        <button on:click={fileErstellen}>File schaffen</button>
    </section>
    <section class="box" on:drop={dropped} on:dragover={allowDrop}>
        <p>Ziehen Sie ihre Aufgabenbuch-Datenbank hier hinein!</p>
    </section>
</div>


<style lang="scss">
.wilkomen-seite {
    display: flex;
    flex-direction: column;
    // display: grid;
    // grid-template-rows: fit-content 1fr;
    height: 100%;
    .message {
    }
    .box {
        margin: .2rem .4rem;
        // width: 100%;
        height: 100%;
        // background-color: gray;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 2px dashed white;
        color: #333;
    }
}
</style>