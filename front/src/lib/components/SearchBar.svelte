<script lang="ts">
	let searchQuery = '';

	function handleSearch(event: Event) {
		event.preventDefault();
		console.log('Buscando:', searchQuery);
	}
</script>

<form on:submit={handleSearch} class="relative mx-auto max-w-2xl">
	<div class="relative">
		<input
			type="text"
			id="search-input"
			bind:value={searchQuery}
			class="peer w-full rounded-lg border-2 border-slate-600 bg-slate-700 px-4 py-3 pr-12 text-sm text-white placeholder-transparent transition duration-300 focus:border-yellow-500
                   focus:outline-none
                   sm:text-base
                   md:text-lg"
			placeholder="Buscar"
		/>
		<label
			for="search-input"
			class="pointer-events-none absolute left-4 top-3 text-xs text-slate-400
                   transition-all
                   peer-placeholder-shown:top-3 peer-placeholder-shown:text-sm
                   peer-focus:top-1 peer-focus:scale-75
                   peer-focus:text-xs peer-focus:opacity-0
                   peer-[:not(:placeholder-shown)]:top-1 peer-[:not(:placeholder-shown)]:scale-75 peer-[:not(:placeholder-shown)]:opacity-0
                   sm:peer-placeholder-shown:text-base
                   md:peer-placeholder-shown:text-lg"
		>
			<span class="block sm:hidden">Buscar</span>
			<span class="hidden sm:block md:hidden">Buscar taxista o placa</span>
			<span class="hidden md:block">Ingrese nombre del taxista o número de placa</span>
		</label>
	</div>
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
</form>
