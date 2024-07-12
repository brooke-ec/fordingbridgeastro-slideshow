<script lang="ts">
	import { fade } from "svelte/transition";
	import type { GalleryEntry } from ".";
	import { FILES_URL } from "./config";

	export let entry: GalleryEntry;
	export let position: ["start" | "end", "start" | "end"];
</script>

<div class="container" style="align-items: {position[1]}; justify-content: {position[0]};" transition:fade>
	<div
		class="box"
		style="border-radius: {position[0] == 'start'
			? '0 10px 10px 0'
			: '10px 0 0 10px'}; box-shadow: {position[0] == 'start' ? '-' : ''}5px 5px 10px black;"
	>
		<h5>{entry.caption}</h5>
		<div class="attribution">
			<img src="{FILES_URL}{entry.member.avatar}" alt="{entry.member.name}'s Avatar" />
			<span>{entry.member.name}</span>
		</div>
	</div>
</div>

<style lang="scss">
	.container {
		position: absolute;
		display: flex;
		height: 100%;
		width: 100%;
	}

	.box {
		background-color: #222222;
		height: fit-content;
		padding: 10px 20px;
		margin: 40px 0;
		z-index: 99999;
		color: white;
	}

	.attribution {
		align-items: center;
		margin-top: 10px;
		display: flex;
		gap: 10px;
	}

	img {
		border-radius: 50%;
		height: 25px;
		width: 25px;
	}
</style>
