<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import DriverCard from '$lib/components/DriverCard.svelte';
	import { fly } from 'svelte/transition';
	import { taxiApps } from '$lib/maps';

	interface Driver {
		id: number;
		name: string;
		license_plate: string;
	}

	interface Complaint {
		id: number;
		driver_id: number;
		taxi_application: string;
		description: string;
		created_at: string;
	}

	interface Image {
		id: number;
		driver_id: number;
		image_url: string;
	}

	interface DriverDetails {
		driver: Driver;
		complaints: {
			items: Complaint[];
			total_items: number;
			page: number;
			per_page: number;
			total_pages: number;
		};
		images: Image[];
	}

	interface SearchResponse {
		items: DriverDetails[];
		total_items: number;
		page: number;
		per_page: number;
		total_pages: number;
	}

	let drivers: DriverDetails[] = [];
	let isLoading = false;
	let error: string | null = null;
	let currentPage = 1;
	let hasMore = true;
	let searchQuery = '';
	let observer: IntersectionObserver;
	let showScrollTop = false;

	const DEFAULT_PHOTO_SVG = `
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full text-gray-400">
            <path fill-rule="evenodd" d="M18.685 19.097A9.723 9.723 0 0021.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 003.065 7.097A9.716 9.716 0 0012 21.75a9.716 9.716 0 006.685-2.653zm-12.54-1.285A7.486 7.486 0 0112 15a7.486 7.486 0 015.855 2.812A8.224 8.224 0 0112 20.25a8.224 8.224 0 01-5.855-2.438zM15.75 9a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" clip-rule="evenodd" />
        </svg>
    `;

	onMount(async () => {
		searchQuery = $page.url.searchParams.get('query') || '';
		if (searchQuery) {
			await fetchDrivers(searchQuery, currentPage);
		} else {
			error = 'No search query provided.';
		}

		setupIntersectionObserver();
	});

	function setupIntersectionObserver() {
		observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting && !isLoading && hasMore) {
					loadMoreDrivers();
				}
			},
			{ threshold: 1 }
		);

		const sentinel = document.querySelector('#sentinel');
		if (sentinel) observer.observe(sentinel);
	}

	async function fetchDrivers(query: string, page: number) {
		isLoading = true;
		error = null;
		try {
			const response = await fetch(
				`http://localhost:4200/api/taxi/drivers/search/with-details?query=${encodeURIComponent(query)}&page=${page}&per_page=10&complaints_page=1&complaints_per_page=5`
			);
			if (!response.ok) {
				throw new Error('Network response was not ok');
			}
			const data: SearchResponse = await response.json();
			drivers = page === 1 ? data.items : [...drivers, ...data.items];
			hasMore = data.page < data.total_pages;
			currentPage = data.page;
		} catch (err) {
			console.error('Error fetching drivers:', err);
			error = 'An error occurred while fetching the data. Please try again.';
		} finally {
			isLoading = false;
		}
	}

	function getReportedApps(complaints: Complaint[]): { name: string; logo: string }[] {
		const uniqueApps = new Set(complaints.map((c) => c.taxi_application));
		return Array.from(uniqueApps).map((app) => {
			const taxiApp = taxiApps.find((ta) => ta.name.toLowerCase() === app.toLowerCase());
			return {
				name: app,
				logo: taxiApp ? taxiApp.icon : '/default-app-logo.png'
			};
		});
	}

	function loadMoreDrivers() {
		if (searchQuery && hasMore) {
			fetchDrivers(searchQuery, currentPage + 1);
		}
	}

	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	function handleScroll() {
		showScrollTop = window.pageYOffset > 300;
	}
</script>

<svelte:head>
	<title>Driver Search Results</title>
</svelte:head>

<svelte:window on:scroll={handleScroll} />

<section class="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 py-12">
	<header class="mb-12 bg-gray-900 py-6">
		<div class="container mx-auto px-4">
			<h1 class="text-center text-4xl font-bold text-yellow-400">Driver Search Results</h1>
			<p class="mt-2 text-center text-gray-400">Showing results for: {searchQuery}</p>
		</div>
	</header>

	<div class="container mx-auto px-4">
		<div class="mx-auto mb-12 max-w-2xl">
			<form on:submit|preventDefault={() => fetchDrivers(searchQuery, 1)} class="flex">
				<input
					bind:value={searchQuery}
					type="text"
					placeholder="Search drivers..."
					class="flex-grow rounded-l-lg bg-gray-700 px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-400"
				/>
				<button
					type="submit"
					class="rounded-r-lg bg-yellow-400 px-6 py-2 text-gray-900 transition duration-300 hover:bg-yellow-300"
				>
					Search
				</button>
			</form>
		</div>

		{#if isLoading && drivers.length === 0}
			<div class="mt-16 flex justify-center">
				<div
					class="h-16 w-16 animate-spin rounded-full border-b-4 border-t-4 border-yellow-400"
				></div>
			</div>
		{:else if drivers.length === 0 && !isLoading}
			<p
				class="mx-auto max-w-md rounded-lg bg-gray-700 bg-opacity-50 p-6 text-center text-lg text-gray-300"
			>
				No drivers found matching "{searchQuery}". Try a different search term.
			</p>
		{:else}
			<div
				class="mx-auto grid max-w-7xl grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
			>
				{#each drivers as driverDetails, index (driverDetails.driver.id)}
					<div
						in:fly={{ y: 20, duration: 400, delay: 200 * (index % 10) }}
						class="flex h-full justify-center p-4"
					>
						<div
							class="h-full w-full max-w-sm overflow-hidden rounded-lg border border-gray-700 bg-gradient-to-br from-gray-800 to-gray-900 shadow-lg transition duration-300 hover:border-yellow-400"
						>
							<DriverCard
								driverData={{
									name: driverDetails.driver.name,
									licensePlate: driverDetails.driver.license_plate,
									reportCount: driverDetails.complaints.total_items,
									reportedApps: getReportedApps(driverDetails.complaints.items),
									photoUrl: driverDetails.images[0]?.image_url || null
								}}
							/>
						</div>
					</div>
				{/each}
			</div>

			{#if isLoading}
				<div class="mt-16 flex justify-center">
					<div
						class="h-12 w-12 animate-spin rounded-full border-b-2 border-t-2 border-yellow-400"
					></div>
				</div>
			{/if}

			{#if hasMore && !isLoading}
				<div class="mt-12 flex justify-center">
					<button
						on:click={loadMoreDrivers}
						class="rounded-lg bg-yellow-400 px-6 py-3 font-semibold text-gray-900 transition duration-300 hover:bg-yellow-300"
					>
						Load More Results
					</button>
				</div>
			{/if}

			<div id="sentinel" class="mt-16 h-4"></div>
		{/if}

		{#if error}
			<div class="mx-auto mt-12 max-w-md rounded-lg bg-red-900 bg-opacity-50 p-6">
				<p class="mb-4 text-center text-lg text-red-400">{error}</p>
				<button
					on:click={() => fetchDrivers(searchQuery, 1)}
					class="w-full rounded bg-red-500 px-4 py-2 text-white transition duration-300 hover:bg-red-600"
				>
					Retry
				</button>
			</div>
		{/if}
	</div>
</section>

{#if showScrollTop}
	<button
		on:click={scrollToTop}
		class="fixed bottom-8 right-8 rounded-full bg-yellow-400 p-3 text-gray-900 shadow-lg transition duration-300 hover:bg-yellow-300"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-6 w-6"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M5 10l7-7m0 0l7 7m-7-7v18"
			/>
		</svg>
	</button>
{/if}
