<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';

	function adjustForMobileSafari() {
		const vh = window.innerHeight * 0.01;
		document.documentElement.style.setProperty('--vh', `${vh}px`);
	}
	onMount(() => {
		adjustForMobileSafari();
		window.addEventListener('resize', adjustForMobileSafari);

		return () => {
			window.removeEventListener('resize', adjustForMobileSafari);
		};
	});
</script>

<slot></slot>
