<script>
	import { wsconnect, sendLocation } from '$lib';
	import { onMount, onDestroy } from 'svelte';

	let locationInterval;

	onMount(() => {
		wsconnect();

		locationInterval = setInterval(async () => {
			navigator.geolocation.getCurrentPosition((position) => {
				const location = {
					latitude: position.coords.latitude,
					longitude: position.coords.longitude
				};
				console.log(location);
				sendLocation(location);
			});
		}, 10 * 1000); // send location every 30 seconds
	});

	onDestroy(() => {
		clearInterval(locationInterval);
	});
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
