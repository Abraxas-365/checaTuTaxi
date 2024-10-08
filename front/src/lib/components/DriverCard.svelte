<script lang="ts">
	import { fly } from 'svelte/transition';
	import { AlertTriangle, Car } from 'lucide-svelte';

	export let driverData: {
		name: string;
		licensePlate: string;
		reportCount: number;
		reportedApps: { name: string; logo: string }[];
		photoUrl: string | null;
	};

	const DEFAULT_PHOTO_SVG = `
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full text-gray-400">
			<path fill-rule="evenodd" d="M18.685 19.097A9.723 9.723 0 0021.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 003.065 7.097A9.716 9.716 0 0012 21.75a9.716 9.716 0 006.685-2.653zm-12.54-1.285A7.486 7.486 0 0112 15a7.486 7.486 0 015.855 2.812A8.224 8.224 0 0112 20.25a8.224 8.224 0 01-5.855-2.438zM15.75 9a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" clip-rule="evenodd" />
		</svg>
	`;
</script>

<div
	class="mx-auto w-full max-w-sm overflow-hidden rounded-2xl border border-gray-700 bg-gray-900 shadow-xl transition-all duration-300 ease-in-out hover:shadow-2xl sm:max-w-md md:max-w-lg"
	in:fly={{ y: 20, duration: 400, delay: 200 }}
>
	<div class="space-y-6 p-5 sm:p-6">
		<div class="flex flex-col items-center space-y-4 sm:flex-row sm:space-x-5 sm:space-y-0">
			<div class="relative h-24 w-24 flex-shrink-0 sm:h-20 sm:w-20">
				<div class="absolute inset-0 rounded-full bg-yellow-400 opacity-20 blur-md"></div>
				<div
					class="relative h-full w-full overflow-hidden rounded-full border-2 border-yellow-400 transition-transform duration-300 ease-in-out hover:scale-105"
				>
					{#if driverData.photoUrl}
						<img
							src={driverData.photoUrl}
							alt={driverData.name}
							class="h-full w-full object-cover"
						/>
					{:else}
						{@html DEFAULT_PHOTO_SVG}
					{/if}
				</div>
			</div>
			<div class="text-center sm:text-left">
				<h2 class="text-2xl font-bold text-white sm:text-xl">{driverData.name}</h2>
				<p class="mt-1 flex items-center justify-center text-yellow-400 sm:justify-start">
					<Car class="mr-2 h-5 w-5" />
					<span class="text-lg font-semibold">{driverData.licensePlate}</span>
				</p>
			</div>
		</div>

		<div
			class="flex flex-col space-y-4 sm:flex-row sm:items-center sm:justify-between sm:space-y-0"
		>
			<div class="flex items-center justify-center text-red-500 sm:justify-start">
				<AlertTriangle class="mr-2 h-6 w-6" />
				<span class="text-lg font-bold">{driverData.reportCount} Quejas</span>
			</div>
			<div class="flex flex-wrap justify-center gap-3 sm:justify-end">
				{#each driverData.reportedApps as app}
					<div
						class="flex h-10 w-10 items-center justify-center rounded-full bg-gray-800 p-1.5 transition-transform duration-300 ease-in-out hover:scale-110"
						title={app.name}
					>
						<img src={app.logo} alt={app.name} class="h-full w-full object-contain" />
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>
