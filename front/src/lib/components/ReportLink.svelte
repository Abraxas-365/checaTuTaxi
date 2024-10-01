<script lang="ts">
	import { Button } from '$lib/components/ui/button';

	function handleReport() {
		// Add your report functionality here
		console.log('Report a taxi driver');
	}
</script>

<Button variant="link" on:click={handleReport} class="text-yellow-400 hover:text-yellow-300">
	¿Necesitas reportar un taxista?
</Button>
