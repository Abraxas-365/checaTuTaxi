<script lang="ts">
	import { fly } from 'svelte/transition';
	import { AlertTriangle, Car } from 'lucide-svelte';

	export let driverData: {
		name: string;
		licensePlate: string;
		reportCount: number;
		reportedApps: { name: string; logo: string }[];
		photoUrl: string;
	};
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
					<img src={driverData.photoUrl} alt={driverData.name} class="h-full w-full object-cover" />
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
