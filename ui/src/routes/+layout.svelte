<script>
	import '../app.css';
	import favicon from '$lib/assets/favicon.png';

	let { children } = $props();
</script>

<svelte:head>
    <title>Fractals</title>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children()}
