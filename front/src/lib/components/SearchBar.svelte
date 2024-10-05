<script lang="ts">
    import { Car } from 'lucide-svelte';
    import { goto } from '$app/navigation';

    interface Driver {
        id: number;
        name: string;
        license_plate: string;
    }

    interface Image {
        id: number;
        driver_id: number;
        image_url: string;
    }

    interface SearchResult {
        driver: Driver;
        images: Image[];
    }

    interface ApiResponse {
        items: SearchResult[];
        total_items: number;
        page: number;
        per_page: number;
        total_pages: number;
    }

    let searchQuery = '';
    let searchResults: SearchResult[] = [];
    let isLoading = false;
    let isFocused = false;

    async function fetchSearchResults(query: string) {
        if (!query) {
            searchResults = [];
            return;
        }

        isLoading = true;
        try {
            const response = await fetch(`http://localhost:4200/api/taxi/drivers/search/with-images?query=${query}&page=1&per_page=10`);
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const data: ApiResponse = await response.json();
            searchResults = data.items || [];
        } catch (error) {
            console.error('Error fetching search results:', error);
            searchResults = [];
        } finally {
            isLoading = false;
        }
    }

    function handleSearch(event: Event) {
        event.preventDefault();
        if (searchQuery.trim()) {
            goto(`/list?query=${encodeURIComponent(searchQuery.trim())}`);
        }
    }

    $: fetchSearchResults(searchQuery);

    function handleFocus() {
        isFocused = true;
    }

    function handleBlur(event: FocusEvent) {
        const relatedTarget = event.relatedTarget as HTMLElement;
        if (relatedTarget && relatedTarget.closest('.search-dropdown')) {
            return;
        }
        isFocused = false;
    }

    function handleDriverSelect(driverId: number) {
        goto(`/${driverId}/driver`);
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === 'Enter' && searchQuery.trim()) {
            event.preventDefault();
            goto(`/list?query=${encodeURIComponent(searchQuery.trim())}`);
        }
    }
</script>

<div class="relative w-full max-w-2xl mx-auto">
    <form on:submit={handleSearch} class="relative">
        <div class="relative">
            <input
                type="text"
                id="search-input"
                bind:value={searchQuery}
                on:input={() => fetchSearchResults(searchQuery)}
                on:focus={handleFocus}
                on:blur={handleBlur}
                on:keydown={handleKeyDown}
                autocomplete="off"
                class="peer w-full rounded-lg border-2 border-yellow-500 bg-slate-800 px-4 py-3 pr-12 text-sm text-white placeholder-transparent transition duration-300 focus:border-yellow-400 focus:outline-none focus:ring-2 focus:ring-yellow-400 focus:ring-opacity-50 sm:text-base md:text-lg"
                placeholder="Buscar"
            />
            <label
                for="search-input"
                class="absolute left-4 top-1 text-xs text-yellow-400 transition-all duration-300 transform origin-left 
               peer-placeholder-shown:text-slate-400 peer-placeholder-shown:top-3 peer-placeholder-shown:text-base peer-placeholder-shown:scale-100
               peer-focus:top-1 peer-focus:text-xs peer-focus:text-yellow-400 peer-focus:scale-75
               peer-[:not(:placeholder-shown)]:top-1 peer-[:not(:placeholder-shown)]:text-xs peer-[:not(:placeholder-shown)]:text-yellow-400 peer-[:not(:placeholder-shown)]:scale-75"
            >
                <span class="block sm:hidden">Buscar</span>
                <span class="hidden sm:block md:hidden">Buscar taxista o placa</span>
                <span class="hidden md:block">Ingrese nombre del taxista o número de placa</span>
            </label>
            <button
                type="submit"
                class="absolute right-2 top-1/2 -translate-y-1/2 rounded-lg bg-yellow-500 p-2 text-slate-900 transition duration-300 hover:bg-yellow-400 focus:outline-none focus:ring-2 focus:ring-yellow-600 focus:ring-offset-2 focus:ring-offset-slate-800"
            >
                <span class="sr-only">Buscar</span>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    class="h-5 w-5"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    />
                </svg>
            </button>
        </div>
        {#if isFocused && (isLoading || searchResults.length > 0)}
            <div class="search-dropdown absolute left-0 right-0 top-full bg-slate-800 border border-yellow-500 rounded-b-lg shadow-lg z-10 max-h-72 overflow-y-auto mt-1 transition-all duration-300 ease-in-out">
                {#if isLoading}
                    <div class="flex items-center justify-center p-4">
                        <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-yellow-500"></div>
                    </div>
                {/if}

                {#if searchResults.length > 0}
                    {#each searchResults as result, index}
                        <div 
                            class="p-3 text-white cursor-pointer hover:bg-slate-700 focus:outline-none focus:bg-slate-700 transition-colors duration-150 ease-in-out {index !== searchResults.length - 1 ? 'border-b border-slate-600' : ''}"
                            on:click={() => handleDriverSelect(result.driver.id)}
                            on:keydown={(e) => e.key === 'Enter' && handleDriverSelect(result.driver.id)}
                            tabindex="0"
                        >
                            <div class="flex items-center">
                                {#if result.images.length > 0}
                                    <img src={result.images[0].image_url} alt={result.driver.name} class="w-12 h-12 rounded-full object-cover mr-3" />
                                {:else}
                                    <div class="bg-yellow-500 rounded-full p-2 mr-3">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-slate-900" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                        </svg>
                                    </div>
                                {/if}
                                <div>
                                    <p class="font-semibold">{result.driver.name}</p>
                                    <div class="flex items-center text-sm text-slate-400">
                                        <Car class="w-4 h-4 mr-1" />
                                        <p>{result.driver.license_plate}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    {/each}
                {:else if searchQuery && !isLoading}
                    <p class="p-3 text-slate-400 text-center">No se encontraron resultados.</p>
                {/if}
            </div>
        {/if}
    </form>
</div>
