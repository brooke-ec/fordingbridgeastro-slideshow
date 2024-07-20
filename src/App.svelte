<script lang="ts">
	import { getConfiguration } from "./lib/specta";
	import Slideshow from "./lib/Slideshow.svelte";
	import { exit } from "@tauri-apps/api/process";
	import { onMount } from "svelte";

	let configuration: Awaited<ReturnType<typeof getConfiguration>> | null = null;
	onMount(async () => (configuration = await getConfiguration()));

	async function close() {
		if (configuration?.mode == "ScreenSaver") await exit(1);
	}
</script>

<svelte:window on:pointermove={close} on:pointerdown={close} on:keydown={close} />

<Slideshow />
