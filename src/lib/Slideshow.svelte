<script lang="ts">
	import Caption from "./Caption.svelte";
	import type { GalleryEntry } from ".";
	import { supabase } from "./supabase";
	import { FILES_URL } from "./config";

	let axis: "width" | "height";
	let bind: HTMLImageElement;
	let position: number = 0;
	let entry: GalleryEntry;
	let animation: number;

	async function next() {
		const count = await supabase.from("gallery").select("*", { count: "exact", head: true });
		const index = Math.floor(Math.random() * count.count!);
		animation = 0;

		const result = await supabase
			.from("gallery")
			.select("maxres, caption, member(name, avatar)")
			.range(index, index);

		entry = result.data![0] as GalleryEntry;
		position = position < 3 ? position + 1 : 0;

		requestAnimationFrame(() => {
			if (bind.complete) loaded();
		});
	}

	async function loaded() {
		let screenRatio = document.body.clientHeight / document.body.clientWidth;
		let imageRatio = bind.clientHeight / bind.clientWidth;

		if (screenRatio - imageRatio > 0) axis = "height";
		else axis = "width";

		animation = Math.floor(Math.random() * 32) + 1;
	}

	next();
</script>

{#if entry}
	{#if position == 0}
		<Caption {entry} position={["start", "start"]} />
	{:else if position == 1}
		<Caption {entry} position={["start", "end"]} />
	{:else if position == 2}
		<Caption {entry} position={["end", "start"]} />
	{:else if position == 3}
		<Caption {entry} position={["end", "end"]} />
	{/if}
{/if}

<div class="container">
	<img
		style="min-{axis}: 125%"
		src="{FILES_URL}{entry?.maxres}"
		class="animation-{animation}"
		on:animationend={next}
		alt={entry?.caption}
		bind:this={bind}
		on:load={loaded}
	/>
</div>

<style lang="scss">
	@use "sass:math";

	.container {
		justify-content: center;
		align-items: center;
		overflow: hidden;
		display: flex;
		height: 100%;
	}

	img {
		animation-timing-function: linear;
		animation-duration: 20s;
		position: relative;
		opacity: 0;
	}

	$loop-max: 32;
	@for $i from 1 through $loop-max {
		$start-x: (math.sin(math.div($i, $loop-max) * math.$pi * 2)) * 12.5;
		$start-y: (math.cos(math.div($i, $loop-max) * math.$pi * 2)) * 12.5;

		.animation-#{$i} {
			animation-name: animation-#{$i};
		}

		@keyframes animation-#{$i} {
			0% {
				right: #{$start-x * 1%};
				top: #{$start-y * 1%};
				opacity: 0;
			}

			15% {
				opacity: 1;
			}

			85% {
				opacity: 1;
			}

			100% {
				right: #{-($start-x * 1%)};
				top: #{-($start-y * 1%)};
				opacity: 0;
			}
		}
	}
</style>
