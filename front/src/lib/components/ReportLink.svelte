<script lang="ts">
    import { Button } from '$lib/components/ui/button';
    import { goto } from '$app/navigation';

    function handleReport() {
        goto('/report');
    }
</script>

<Button variant="link" on:click={handleReport} class="text-yellow-400 hover:text-yellow-300">
    ¿Necesitas reportar un taxista?
</Button>
