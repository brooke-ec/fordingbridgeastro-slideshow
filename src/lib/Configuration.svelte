<script>
	import { faGlobe, faExclamationTriangle, faArrowCircleRight } from "@fortawesome/free-solid-svg-icons";
	import { faGithub, faDiscord } from "@fortawesome/free-brands-svg-icons";
	import { startSlideshow, installScreensaver } from "./specta";
	import { exit } from "@tauri-apps/api/process";
	import heroSrc from "./hero.webp";
	import Logo from "./Logo.svelte";
	import Fa from "svelte-fa";

	async function start() {
		await startSlideshow();
		await exit(0);
	}
</script>

<main>
	<div class="title">
		<Logo height="100" width="100" variant="light" />
		<h1><span>Slideshow</span><span>Gallery</span><span>Astonomers</span><span>Fordingbridge</span></h1>
	</div>
	<p>Images may be subject to copyright.</p>
	<div class="hero" style="background-image: url({heroSrc});" />
	<ul>
		<a href="https://fordingbridgeastro.org.uk/" target="_blank"><Fa icon={faGlobe} /> Homepage</a>
		<a href="https://discord.gg/t8Y6KMZTfD" target="_blank"><Fa icon={faDiscord} /> Discord Server</a>
		<a href="https://github.com/brooke-ec/fordingbridgeastro-slideshow/issues" target="_blank">
			<Fa icon={faExclamationTriangle} /> Issue Tracker
		</a>
		<a href="https://github.com/brooke-ec/fordingbridgeastro-slideshow" target="_blank">
			<Fa icon={faGithub} /> Source
		</a>
	</ul>
	<div class="actions">
		<button on:click={start}>Start Slideshow <Fa icon={faArrowCircleRight} /></button>
		<button on:click={installScreensaver}>Install Screensaver <Fa icon={faArrowCircleRight} /></button>
	</div>
</main>

<style lang="scss">
	main {
		background-color: #222222;
		padding: 10px;
		height: 100%;
	}

	.title {
		align-items: center;
		display: flex;
		gap: 10px;

		h1 {
			flex-direction: row-reverse;
			flex-wrap: wrap-reverse;
			justify-content: start;
			display: flex;
			gap: 0.3em;
		}
	}

	.hero {
		background-size: cover;
		background-position-y: 50%;
		border-radius: 10px;
		margin: 20px 0;
		height: 100px;
	}

	ul {
		flex-direction: column;
		margin-top: 20px;
		font-size: 18px;
		display: flex;
		gap: 10px;

		* {
			width: fit-content;
		}
	}

	.actions {
		flex-direction: column;
		position: absolute;
		align-items: end;
		display: flex;
		bottom: 10px;
		gap: 10px;
		right: 0;

		button {
			transition: background-color 100ms;
			border-radius: 10px 0 0 10px;
			background-color: #cf2c2d;
			padding: 5px 5px 5px 20px;
			align-items: center;
			width: fit-content;
			font-size: 23px;
			display: flex;
			color: white;
			border: none;
			gap: 10px;

			&:hover {
				background-color: #a82020;
			}

			&:active {
				background-color: #880f0f;
			}
		}
	}
</style>
