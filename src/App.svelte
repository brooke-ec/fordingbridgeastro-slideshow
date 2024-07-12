<script>
	import Slideshow from "./lib/Slideshow.svelte";
	import { exit } from "@tauri-apps/api/process";

	async function close() {
		if (import.meta.env.MODE != "development") await exit(1);
	}
</script>

<svelte:window on:pointermove={close} on:pointerdown={close} on:keydown={close} />

<Slideshow />
