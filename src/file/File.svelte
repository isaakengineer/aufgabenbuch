<script>
	import { event } from "@tauri-apps/api";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";

	import i18n from "$lib/i18n";

	let file;
	let pfad = null;

	let sprachen = [
		{ id: "de", name: "Deutsch" },
		{ id: "fr", name: "Français" },
		{ id: "it", name: "italiano" },
		{ id: "zh", name: "中文" },
		{ id: "pt", name: "Português" },
		{ id: "ru", name: "русский" },
		{ id: "hi", name: "हिंदी" },
		{ id: "es", name: "Español" },
		{ id: "nl", name: "Dutch" },
		// { id: "pl", name: "Polski" },
		// { id: "ro", name: "Română" },
		// { id: "tr", name: "Türkçe" },
		// { id: "uk", name: "Українська" },
		{ id: "ja", name: "日本語" },
		// { id: "ko", name: "한국어" },
		// { id: "ar", name: "العربية" },
		{ id: "sr-Cyrl", name: "Српски (cyrillic)" },
		{ id: "sr", name: "Srpski (latin)" },
		{ id: "en", name: "English (Int)" },
	];
	let sprache = $i18n.language;

	// Ein Funktionen-Paar, weil ansonsten Tauri kann nicht durch Browser die Pfad von "drag-drop event" lesen.
	listen("tauri://drag-enter", async (event) => {
		console.log("drag enter event", event);
		pfad = event.payload.paths[0];
	});
	listen("tauri://drag-leave", (event) => {
		console.log("dragged file left!");
		pfad = null;
	});
	listen("tauri://drag-drop", (event) => {
		console.log("tauri drop event!");
		console.log(event);
		pfad = event.payload.paths[0];
		pfadLesen(event);
	});

	window.addEventListener(
		"drop",
		function (e) {
			e = e || event;
			e.preventDefault();
		},
		false,
	); //preventing drag and drop nonesense!

	async function file_waehlen() {
		file = await invoke("file_waehlen");
	}

	const fileErstellen = async () => {
		file = await invoke("file_erstellen");
		console.log(file);
	};

	const pfadLesen = async (event) => {
		if (pfad) {
			file = await invoke("dateipfad_eingegeben", {
				pfad: pfad, // hier die Pfad wird durch von Tauri festgelegten Data erfüllt
			});
		} else {
			console.log("something went wrong during drop!", event);
		}
	};

	const allowDrop = (event) => {
		// nur zum Testen
		// console.log("something!")
	};
	function dragoverHandler(ev) {
		ev.preventDefault();
	}
</script>

<div class="wilkomen-seite">
	<section class="message">
		<h1>{$i18n.t("abbild.willkommen.kurz")}</h1>
	</section>
	<section class="message">
		<label for="sprache">{$i18n.t("sprache-ändern")}</label>
		<select
			name="sprache"
			bind:value={sprache}
			on:change={() => {
				console.log("sprache ändern", sprache);
				$i18n.changeLanguage(sprache);
			}}
		>
			{#each sprachen as sprache}
				<option value={sprache.id}>{sprache.name}</option>
			{/each}
		</select>
	</section>
	<section class="message">
		<p>{$i18n.t("abbild.willkommen.status")}</p>
		<p>{$i18n.t("abbild.willkommen.ratschlag")}</p>
		<button on:click={file_waehlen}>{$i18n.t("datei.wählen")}</button>
		<button on:click={fileErstellen}>{$i18n.t("datei.erstellen")}</button>
	</section>
	<div
		class="box dropzone"
		on:drop={pfadLesen}
		on:dragover={allowDrop}
		on:dragover={dragoverHandler}
	>
		<p>{$i18n.t("abbild.willkommen.dropzone")}</p>
	</div>
	<section class="message">
		<p>
			<span
				>{$i18n.t("gebrauchhinweise.aktualisierung")}
				<a href="https://aufgabenbuch.techne.schloosser.com/"
					>{$i18n.t("gebrauchhinweise.ort")}</a
				></span
			>
		</p>
		<p>{$i18n.t("dedication", { ns: "a" })}</p>
		<p>
			<span
				><a href="https://aufgabenbuch.techne.schloosser.com/"
					>{$i18n.t("gebrauchhinweise.webseite")}</a
				></span
			>
			<span>|</span>
			<span
				><a href="https://aufgabenbuch.techne.schloosser.com/"
					>{$i18n.t("gebrauchhinweise.handbuch")}</a
				></span
			>
			<span>|</span>
			<span
				><a href="https://aufgabenbuch.techne.schloosser.com/"
					>{$i18n.t("gebrauchhinweise.lizenz")}</a
				></span
			>
			<span>|</span>
			<span>{$i18n.t("gebrauchhinweise.herkunft")}</span>
		</p>
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
			margin: 0.2rem 0.4rem;
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
