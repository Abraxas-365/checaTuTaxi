<script lang="ts">
    import { onMount } from 'svelte';
    import { fly } from 'svelte/transition';
    import { page } from '$app/stores';
    import CardQueja from '$lib/components/CardQueja.svelte';

    interface Driver {
        id: number;
        name: string;
        license_plate: string;
    }

    interface Complaint {
        id: number;
        driver_id: number;
        location_id: number;
        taxi_application: string;
        description: string;
      created_at: string;
    }

    interface ApiResponse {
        driver: Driver;
        complaints: {
            items: Complaint[];
            total_items: number;
            page: number;
            per_page: number;
            total_pages: number;
        };
        images: string[];
    }

    $: id = $page.params.id;

    let driverData: ApiResponse | null = null;
    let loading = true;

    async function fetchDriverData(driverId: string) {
        try {
            const response = await fetch(`http://localhost:4200/api/taxi/driver/${driverId}/details?page=1&per_page=10`);
            if (!response.ok) {
                throw new Error('Failed to fetch driver data');
            }
            const data: ApiResponse = await response.json();
            return data;
        } catch (error) {
            console.error('Error fetching driver data:', error);
            return null;
        }
    }

    function formatDate(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleString('es-ES', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
        });
    }

    $: {
        loading = true;
        fetchDriverData(id).then((data) => {
            driverData = data;
            loading = false;
        });
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
                            src={driverData.images[0] || `https://via.placeholder.com/300x300.png?text=${driverData.driver.name}`}
                            alt={driverData.driver.name}
                            class="h-full w-full object-cover"
                        />
                    </div>
                    <div class="flex-1 text-center md:text-left">
                        <h2 class="mb-4 text-3xl font-bold text-white md:text-4xl">{driverData.driver.name}</h2>
                        <p class="mb-4 text-lg text-yellow-300">License Plate: {driverData.driver.license_plate}</p>
                    </div>
                </div>

                <h3 class="mb-4 text-2xl font-semibold text-white">
                    Quejas <span class="text-yellow-300">({driverData.complaints.total_items})</span>
                </h3>
                <div
                    class="flex flex-1 flex-col overflow-hidden rounded-2xl bg-slate-800 bg-opacity-50 p-4 shadow-xl backdrop-blur-sm md:p-6"
                >
                    <div class="flex-1 overflow-y-auto pr-2">
                        <div class="space-y-4">
                            {#each driverData.complaints.items as complaint}
                                <CardQueja 
                                    app={{ name: complaint.taxi_application, logo: `https://via.placeholder.com/50x50.png?text=${complaint.taxi_application}` }}
                                    complaint={complaint.description}
                                    date={formatDate(complaint.created_at)}
                                />
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
