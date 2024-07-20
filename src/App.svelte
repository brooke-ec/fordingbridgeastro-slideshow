<script lang="ts">
	import { getCurrent } from "@tauri-apps/api/window";
	import { getConfiguration } from "./lib/specta";
	import Slideshow from "./lib/Slideshow.svelte";
	import { exit } from "@tauri-apps/api/process";
	import { onMount } from "svelte";

	let configuration: Awaited<ReturnType<typeof getConfiguration>> | null = null;
	onMount(async () => (configuration = await getConfiguration()));

	async function interact() {
		if (configuration?.mode == "ScreenSaver") await exit(1);
	}

	async function keydown(e: KeyboardEvent) {
		interact();

		if (configuration?.mode == "SlideShow") {
			const window = getCurrent();

			if (e.key == "F11") await window.isFullscreen().then((f) => window.setFullscreen(!f));
			if (e.key == "Escape") await window.setFullscreen(false);
		}
	}
</script>

<svelte:window on:pointermove={interact} on:pointerdown={interact} on:keydown={keydown} />

<Slideshow />
