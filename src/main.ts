import "greset";
import "./lib/global.scss";
import App from "./App.svelte";

const app = new App({
	target: document.getElementById("app")!,
});

export default app;
