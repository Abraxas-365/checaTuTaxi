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

    const DEFAULT_PHOTO_URL = '/default-driver-photo.png';

    onMount(async () => {
        searchQuery = $page.url.searchParams.get('query') || '';
        if (searchQuery) {
            await fetchDrivers(searchQuery, currentPage);
        } else {
            error = "No search query provided.";
        }

        setupIntersectionObserver();
    });

    function setupIntersectionObserver() {
        observer = new IntersectionObserver((entries) => {
            if (entries[0].isIntersecting && !isLoading && hasMore) {
                loadMoreDrivers();
            }
        }, { threshold: 1 });

        const sentinel = document.querySelector('#sentinel');
        if (sentinel) observer.observe(sentinel);
    }

    async function fetchDrivers(query: string, page: number) {
        isLoading = true;
        error = null;
        try {
            const response = await fetch(`http://localhost:4200/api/taxi/drivers/search/with-details?query=${encodeURIComponent(query)}&page=${page}&per_page=10&complaints_page=1&complaints_per_page=5`);
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const data: SearchResponse = await response.json();
            drivers = [...drivers, ...data.items];
            hasMore = data.page < data.total_pages;
            currentPage = data.page;
        } catch (err) {
            console.error('Error fetching drivers:', err);
            error = "An error occurred while fetching the data. Please try again.";
        } finally {
            isLoading = false;
        }
    }

    function getReportedApps(complaints: Complaint[]): { name: string; logo: string }[] {
        const uniqueApps = new Set(complaints.map(c => c.taxi_application));
        return Array.from(uniqueApps).map(app => {
            const taxiApp = taxiApps.find(ta => ta.name.toLowerCase() === app.toLowerCase());
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
</script>

<svelte:head>
    <title>Driver Search Results</title>
</svelte:head>

<section class="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 py-12">
    <div class="container mx-auto px-4">
        <h1 class="text-4xl font-bold mb-12 text-center text-white">Driver Search Results</h1>
        
        {#if error}
            <p class="text-red-400 text-center text-lg bg-red-900 bg-opacity-50 rounded-lg p-4 max-w-md mx-auto">{error}</p>
        {:else if drivers.length === 0 && !isLoading}
            <p class="text-gray-300 text-center text-lg bg-gray-700 bg-opacity-50 rounded-lg p-4 max-w-md mx-auto">No drivers found.</p>
        {:else}
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-7xl mx-auto">
                {#each drivers as driverDetails, index (driverDetails.driver.id)}
                    <div 
                        in:fly={{ y: 20, duration: 400, delay: 200 * (index % 10) }} 
                        class="flex justify-center p-4"
                    >
                        <div class="w-full max-w-sm bg-gray-800 rounded-lg shadow-lg overflow-hidden">
                            <DriverCard
                                driverData={{
                                    name: driverDetails.driver.name,
                                    licensePlate: driverDetails.driver.license_plate,
                                    reportCount: driverDetails.complaints.total_items,
                                    reportedApps: getReportedApps(driverDetails.complaints.items),
                                    photoUrl: driverDetails.images[0]?.image_url || DEFAULT_PHOTO_URL
                                }}
                            />
                        </div>
                    </div>
                {/each}
            </div>
            
            {#if isLoading}
                <div class="flex justify-center mt-16">
                    <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-yellow-400"></div>
                </div>
            {/if}
            
            <div id="sentinel" class="h-4 mt-16"></div>
        {/if}
    </div>
</section>
