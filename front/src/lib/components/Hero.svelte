<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { onMount } from 'svelte';
	import { ChevronDown } from 'lucide-svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import ReportLink from '$lib/components/ReportLink.svelte';

	let visible = false;

	onMount(() => {
		visible = true;
	});

	function scrollToInfo() {
		const infoSection = document.getElementById('info-section');
		if (infoSection) {
			infoSection.scrollIntoView({ behavior: 'smooth' });
		}
	}
</script>

{#if visible}
	<section
		class="full-height relative flex flex-col justify-center px-4 text-center sm:px-6"
		in:fade={{ duration: 300 }}
	>
		<div class="container relative z-10 mx-auto">
			<div
				class="mb-8 space-y-4 sm:mb-12 sm:space-y-6 md:mb-16 md:space-y-8"
				in:fly={{ y: 20, duration: 400, delay: 300 }}
			>
				<h1
					class="text-4xl font-bold tracking-tight text-white sm:text-5xl md:text-6xl lg:text-7xl xl:text-8xl"
				>
					Checa Tu Taxi
				</h1>
				<p
					class="mx-auto max-w-sm text-base font-normal text-slate-300 sm:max-w-md sm:text-lg md:max-w-lg md:text-xl lg:max-w-2xl lg:text-2xl"
				>
					Verifica la seguridad de tu viaje. Busca información sobre un taxista por <span
						class="font-semibold text-yellow-400">Nombre</span
					>
					o <span class="font-semibold text-yellow-400">Placa</span>.
				</p>
			</div>
			<div
				in:fly={{ y: 20, duration: 400, delay: 600 }}
				class="mx-auto w-full max-w-xs sm:max-w-md md:max-w-lg lg:max-w-2xl xl:max-w-3xl"
			>
				<SearchBar />
				<div class="mt-4 text-right sm:mt-6">
					<ReportLink />
				</div>
			</div>
		</div>
		<div class="absolute bottom-4 left-0 w-full text-center sm:bottom-6 md:bottom-8">
			<button
				on:click={scrollToInfo}
				class="text-yellow-400 transition-colors duration-300 hover:text-yellow-300 focus:outline-none"
				aria-label="Scroll to more information"
			>
				<ChevronDown class="h-8 w-8 animate-bounce sm:h-10 sm:w-10 md:h-12 md:w-12" />
			</button>
		</div>
	</section>
{/if}
