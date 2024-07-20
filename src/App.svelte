<script lang="ts">
	import { getConfiguration, type Mode } from "./lib/specta";
	import Configuration from "./lib/Configuration.svelte";
	import { getCurrent } from "@tauri-apps/api/window";
	import Slideshow from "./lib/Slideshow.svelte";
	import { exit } from "@tauri-apps/api/process";
	import { onMount } from "svelte";

	let mode: Mode | null = null;
	onMount(() => getConfiguration().then((c) => (mode = c.mode)));

	async function interact() {
		if (mode == "ScreenSaver") await exit(0);
	}

	async function keydown(e: KeyboardEvent) {
		interact();

		if (mode == "SlideShow") {
			const window = getCurrent();

			if (e.key == "F11") await window.isFullscreen().then((f) => window.setFullscreen(!f));
			if (e.key == "Escape") await window.setFullscreen(false);
		}
	}
</script>

<svelte:window on:pointermove={interact} on:pointerdown={interact} on:keydown={keydown} />

{#if mode == "ScreenSaver" || mode == "SlideShow"}
	<Slideshow />
{:else if mode == "Configuration"}
	<Configuration />
{/if}
