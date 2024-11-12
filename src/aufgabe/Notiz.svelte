<script>
	import i18n from '$lib/i18n';
	import { Aufgabe, updateStore } from './store.js';
</script>

<textarea
	name="notiz"
	bind:value={$Aufgabe.notiz}
	placeholder="Notiz"
	rows="4" />

<style lang="scss">
textarea {
	resize: none;
}
</style>
