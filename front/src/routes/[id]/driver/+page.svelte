<script lang="ts">
	import CardQueja from '$lib/components/CardQueja.svelte';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import { page } from '$app/stores';

	$: id = $page.params.id;

	// Fake data generator function
	function generateFakeData(id: string) {
		const apps = [
			{
				name: 'Uber',
				logo: 'https://upload.wikimedia.org/wikipedia/commons/thumb/5/58/Uber_logo_2018.svg/2560px-Uber_logo_2018.svg.png'
			},
			{
				name: 'Lyft',
				logo: 'https://upload.wikimedia.org/wikipedia/commons/thumb/a/a0/Lyft_logo.svg/2560px-Lyft_logo.svg.png'
			}
		];

		const complaintTexts = [
			'Driver was rude and unprofessional.',
			'Vehicle was not in good condition.',
			'Driver took a longer route than necessary.',
			'Driver cancelled the ride at the last minute.',
			'Driver was texting while driving.'
		];

		const reportCount = 10;

		const complaints = Array.from({ length: reportCount }, () => {
			const app = apps[Math.floor(Math.random() * apps.length)];
			return {
				app: app,
				text: complaintTexts[Math.floor(Math.random() * complaintTexts.length)],
				date: new Date(Date.now() - Math.random() * 10000000000).toLocaleDateString()
			};
		});

		const reportedApps = [...new Set(complaints.map((c) => c.app))];

		return {
			id: id,
			name: `Driver ${id}`,
			licensePlate: `ABC-${id}23`,
			reportCount: reportCount,
			reportedApps: reportedApps,
			photoUrl: `https://randomuser.me/api/portraits/men/${parseInt(id) % 100}.jpg`,
			complaints: complaints
		};
	}

	let driverData: any;
	let loading = true;

	$: {
		loading = true;
		// Simulate API call delay
		setTimeout(() => {
			driverData = generateFakeData(id);
			loading = false;
		}, 1000);
	}
</script>

{#if loading}
	<div class="flex h-screen items-center justify-center">
		<p class="text-xl text-white">Loading...</p>
	</div>
{:else if driverData}
	<div class="h-screen p-4 md:p-8" in:fly={{ y: 20, duration: 400, delay: 200 }}>
		<div class="container mx-auto h-full max-w-4xl">
			<div class="flex h-full flex-col">
				<div class="mb-8 flex flex-col items-center md:flex-row md:items-start md:space-x-8">
					<div
						class="mb-6 h-48 w-48 flex-shrink-0 overflow-hidden rounded-lg border-4 border-slate-700 shadow-lg md:mb-0 md:h-64 md:w-64"
					>
						<img
							src={driverData.photoUrl}
							alt={driverData.name}
							class="h-full w-full object-cover"
						/>
					</div>
					<div class="flex-1 text-center md:text-left">
						<h2 class="mb-4 text-3xl font-bold text-white md:text-4xl">{driverData.name}</h2>
						<p class="mb-4 text-lg text-yellow-300">License Plate: {driverData.licensePlate}</p>
						<div class="flex flex-wrap justify-center gap-4 md:justify-start">
							{#each driverData.reportedApps as app}
								<div
									class="h-14 w-14 rounded-full bg-slate-700 p-2 shadow-md transition-transform hover:scale-110"
									title={app.name}
								>
									<img src={app.logo} alt={app.name} class="h-full w-full object-contain" />
								</div>
							{/each}
						</div>
					</div>
				</div>

				<h3 class="mb-4 text-2xl font-semibold text-white">
					Quejas <span class="text-yellow-300">({driverData.reportCount})</span>
				</h3>
				<div
					class="flex flex-1 flex-col overflow-hidden rounded-2xl bg-slate-800 bg-opacity-50 p-4 shadow-xl backdrop-blur-sm md:p-6"
				>
					<div class="flex-1 overflow-y-auto pr-2">
						<div class="space-y-4">
							{#each driverData.complaints as complaint}
								<CardQueja app={complaint.app} complaint={complaint.text} date={complaint.date} />
							{/each}
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{:else}
	<div class="flex h-screen items-center justify-center">
		<p class="text-xl text-white">No driver data available.</p>
	</div>
{/if}
