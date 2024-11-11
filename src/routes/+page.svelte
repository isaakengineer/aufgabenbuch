<script>
	import i18n from '$lib/i18n';
	import { isLoading } from 'svelte-i18next';

	import { invoke } from "@tauri-apps/api/core";
	import { emit, once, listen } from "@tauri-apps/api/event";
	import { getCurrentWindow } from '@tauri-apps/api/window';

	import Aufgabe from "../aufgabe/Aufgabe.svelte";
	import Liste from "../liste/Liste.svelte";
	import Buch from "../liste/Buch.svelte";
	import Gerade from "../liste/Gerade.svelte";
	import Eingangskorb from '../liste/Eingangskorb.svelte';
	import { liste, gruppen } from "../liste/store.js";

	import File from "../file/File.svelte";
	import { Aussehen, Ausstattung } from "./store.js";

	import { Tag, XCircle } from "phosphor-svelte";

	let aufgabenList = $liste;

	let name = "";
	let greetMsg = "";
	let ausstatung = {
		haupt: "nichts",
	};
	let list = {};
	let file = {};

	let elementalCloseButtonWeight = "duotone";

	const haupt = [
		"liste",
		"gerade",
		// "suche", // TODO
		// "gruppen", // Fähigkeit von TaskLog, ob es sinn macht oder nicht ??
		"eingangskorb",
		"buch",
		// "schliessen", // Extra Fähigkeit, vielleicht nicht nötig
	];

	// let aufgabenList = [];

	const appWindow = getCurrentWindow();

	listen("file-gewaehlt", async (event) => {
		$Ausstattung.identitaet = event.payload;
		$Ausstattung.haupt = "liste";
		console.log($Ausstattung.identitaet);
		$liste = await invoke("list_alle");
		$gruppen = await invoke("gruppen_alle");
	});

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await invoke("greet", { name });
		// invoke("greet", { name });
	}

	async function controlle(item) {
		console.log(item);
		switch (item) {
			case "eingangskorb":
				$Ausstattung.haupt = "eingangskorb";
				$liste = await invoke("prioritaetenliste", { prioritaet: $Ausstattung.gezeigtePrioritaet });
				console.log($liste);
				break;
			case "gerade":
				console.log("auf GERADE wechseln");
				$Ausstattung.haupt = "gerade";
				$liste = await invoke("list_jetzige");
				break;
			case "liste":
				console.log("liste aktualizieren");
				$Ausstattung.haupt = "liste";
				$liste = await invoke("list_alle");
				console.log("fetch gruppen");
				$gruppen = await invoke("gruppen_alle");
				console.log($gruppen);
				break;
			case "buch":
				$Ausstattung.haupt = "buch";
				$liste = await invoke("list_erledigt");
			default:
				break;
		}
	}

	const gruppenZeigenToggle = async function() {
		switch ($Ausstattung.gruppenZeigen) {
			case false:
				$gruppen = await invoke("gruppen_alle");
			case true:
				$Ausstattung.gruppenZeigen = !$Ausstattung.gruppenZeigen
		}
	}
</script>

{#if $isLoading}
	<p>loading...</p>
{:else}
	<div class="app">
		<header>
			{#if $Ausstattung.haupt !== "nichts"}
				<div class="name">{$Ausstattung.identitaet.name}</div>
				{#if $Ausstattung.haupt === "liste"}
					<div class="titel">Aufgaben Liste</div>
					<div class="action">
						<a
							class="ausweiten aktion"
							on:click={gruppenZeigenToggle} >
							{#if $Ausstattung.gruppenZeigen}
								<Tag weight="fill" size="1.5em" />
							{:else}
								<Tag weight="duotone" size="1.5em" />
							{/if}
						</a>
					</div>
				{:else if $Ausstattung.haupt === "gerade"}
					<div class="titel">{ $i18n.t('gerade') }</div>
				{/if}
			{/if}
			<div data-tauri-drag-region class="titlebar windowdragger">
			</div>
		  <div>
				<button class="elemental titlebar-button" id="titlebar-close"
					on:mouseover={() => elementalCloseButtonWeight = "fill"}
					on:mouseleave={() => elementalCloseButtonWeight = "duotone"}
					on:click={() => appWindow.close()}>
					<XCircle size="1.5em" bind:weight={elementalCloseButtonWeight} />
				</button>
		  </div>
		</header>
		<main>
			{#if $Ausstattung.haupt === "nichts"}
				<File />
			{:else if $Ausstattung.haupt === "liste"}
				<Liste />
			{:else if $Ausstattung.haupt === "gerade"}
				<Gerade />
			{:else if $Ausstattung.haupt === "buch"}
				<Buch />
			{:else if $Ausstattung.haupt === "eingangskorb"}
				<Eingangskorb />
			{/if}
		</main>
		<aside>
			<Aufgabe deaktiviert={$Ausstattung.haupt === "nichts"} />
		</aside>
		<footer>
			<nav class="kontrollen">
				{#each haupt as item}
					<a href="#" on:click={() => controlle(item)}>{ $i18n.t(item) }</a>
				{/each}
			</nav>
		</footer>
	</div>
{/if}

<style lang="scss">
#titlebar-close {
	color: indianred;
}
	.ausweiten {
		/* display: block; */
	}
	:global(a.aktion) {
		display: flex;
		align-content: center;
		cursor: pointer;
	}
	:global(a.aktion > svg) {
		padding: 0.4rem;
	}
	:global(.debug) {
		color: #333;
		background-color: #bcd8db;
		// margin: .2em;
		padding: 0.2em;
		box-sizing: border-box;
	}
	:global(section.message) {
		padding: 0.5rem 1rem;
	}
	nav.kontrollen {
		display: flex;
		justify-content: space-around;
		a {
			display: block;
			padding: 0.5rem;
			padding-bottom: 0;
			color: #333;
			text-decoration: none;
			text-transform: capitalize;
			position: relative;
			&:hover {
				color: #000;
				&::before {
					transform: scaleX(1);
				}
			}
			&::before {
				content: "";
				position: absolute;
				display: block;
				width: 100%;
				height: 2px;
				bottom: 0;
				left: 0;
				background-color: #000;
				transform: scaleX(0);
				transition: transform 0.3s ease;
			}
		}
	}
	.app {
		box-sizing: border-box;
		padding: 1px;
		display: grid;
		grid-template: "header" "." "main" "." "aside" "." "footer";
		grid-template-rows: 2.8rem 1px 5fr 1px 1fr 1px 2.8rem;
		flex-direction: column;
		background-color: black;
		height: 100vh;
		width: 100vw;
		> aside {
			grid-area: aside;
			background-color: #eee;
		}
		> header {
			grid-area: header;
		}
		> footer {
			grid-area: footer;
			background-color: #fff;
		}
		> main {
			grid-area: main;
			background-color: #f6f6f6;
			overflow-y: auto;
		}
	}

	.app > header {
		background-color: #fff;
		display: flex;
		> div {
			align-content: center;
			button.elemental {
				border: none;
				box-shadow: none;
				margin: .2rem;
				align-content: center;
				display: flex;
				&:hover {
					box-shadow: none;
				}
			}
		}
		> .windowdragger {
			flex: 1;
			cursor: move;
		}
		> .name {
			padding: 0.6rem 1rem;
			font-size: 1.1rem;
			font-family: "LatoWebBold";
			text-transform: capitalize;
		}
		> .titel {
			padding: 0.6rem 1rem;
			color: #333;
		}
	}
	.app > main > .container {
		background-color: gray;
	}

	.logo.vite:hover {
		filter: drop-shadow(0 0 2em #747bff);
	}

	.logo.svelte-kit:hover {
		filter: drop-shadow(0 0 2em #ff3e00);
	}

	:root {
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
		font-size: 16px;
		line-height: 24px;
		font-weight: 400;

		color: #0f0f0f;
		background-color: #f6f6f6;

		font-synthesis: none;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		-webkit-text-size-adjust: 100%;
	}

	.container {
		margin: 0;
		padding-top: 10vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		text-align: center;
	}

	.logo {
		height: 6em;
		padding: 1.5em;
		will-change: filter;
		transition: 0.75s;
	}

	.logo.tauri:hover {
		filter: drop-shadow(0 0 2em #24c8db);
	}

	.row {
		display: flex;
		justify-content: center;
	}

	a {
		font-weight: 500;
		color: #646cff;
		text-decoration: inherit;
	}

	a:hover {
		color: #535bf2;
	}

	h1 {
		text-align: center;
	}

	input,
	button {
		border-radius: 8px;
		border: 1px solid transparent;
		padding: 0.6em 1.2em;
		font-size: 1em;
		font-weight: 500;
		font-family: inherit;
		color: #0f0f0f;
		background-color: #ffffff;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}

	button {
		cursor: pointer;
	}

	button:hover {
		border-color: #396cd8;
	}
	button:active {
		border-color: #396cd8;
		background-color: #e8e8e8;
	}

	input,
	button {
		outline: none;
	}

	#greet-input {
		margin-right: 5px;
	}

	@media (prefers-color-scheme: dark) {
		:root {
			color: #f6f6f6;
			background-color: #2f2f2f;
		}

		a:hover {
			color: #24c8db;
		}

		input,
		button {
			color: #ffffff;
			background-color: #0f0f0f98;
		}
		button:active {
			background-color: #0f0f0f69;
		}
	}
</style>
