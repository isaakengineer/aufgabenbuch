<script>
	import { event } from "@tauri-apps/api";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";

	let file;
	let pfad = null;

	// Ein Funktionen-Paar, weil ansonsten Tauri kann nicht durch Browser die Pfad von "drag-drop event" lesen.
  listen('tauri://drag-enter', async (event) => {
    console.log("drag enter event", event);
    pfad = event.payload.paths[0];
  });
  listen('tauri://drag-leave', event => {
  	console.log("dragged file left!")
    pfad = null;
  });
  listen('tauri://drag-drop', (event) => {
  console.log("tauri drop event!")
  	console.log(event);
  })

 	window.addEventListener("drop",function(e){
	  e = e || event;
	  e.preventDefault();
	},false); //preventing drag and drop nonesense!

	async function file_waehlen() {
	    file = await invoke("file_waehlen");
	}
	const fileErstellen = async () => {
	    file = await invoke("file_erstellen");
	    console.log(file);
	}

	const pfadLesen = async (event) => {
  	if (pfad) {
	  	file = await invoke("dateipfad_eingegeben", {
				pfad: pfad // hier die Pfad wird durch von Tauri festgelegten Data erfüllt
		  });
   	} else {
    	console.log("something went wrong during drop!", event);
    }
  }

	const allowDrop = (event) => { // nur zum Testen
		// console.log("something!")
	}
	function dragoverHandler(ev) {
	  ev.preventDefault();
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
    <div class="box dropzone" on:drop={pfadLesen} on:dragover={allowDrop} on:dragover={dragoverHandler} >
        <p>Ziehen Sie ihre Aufgabenbuch-Datenbank hier hinein!</p>
    </div>
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
