<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { onMount } from 'svelte';
	import { locations, taxiApps } from '$lib/maps';
	import { Camera, Upload, X, CheckCircle, AlertCircle, Search } from 'lucide-svelte';
	import debounce from 'lodash/debounce';
	import { goto } from '$app/navigation';

	let visible = false;
	let formData: any = {
		description: '',
		taxi_driver_name: '',
		taxi_license_plate: '',
		location_id: 0,
		taxi_application: '',
		driver_image: '',
		complaint_images: []
	};

	let isSubmitting = false;
	let submitSuccess = false;
	let submitError = '';

	let driverImageFile: File | null = null;
	let complaintImageFiles: File[] = [];

	let driverSuggestions: any[] = [];
	let showSuggestions = false;

	let locationSearch = '';
	let filteredLocations: typeof locations = [];
	let showLocationSuggestions = false;

	onMount(() => {
		visible = true;
	});

	const searchDrivers = debounce(async (query: string) => {
		if (query.length < 2) {
			driverSuggestions = [];
			showSuggestions = false;
			return;
		}

		try {
			const response = await fetch(
				`http://localhost:4200/api/taxi/drivers/search?query=${encodeURIComponent(query)}&page=1&per_page=5`
			);
			if (response.ok) {
				const data = await response.json();
				console.log('Raw API response:', data);

				if (data && data.items && Array.isArray(data.items)) {
					driverSuggestions = data.items;
				} else {
					console.error('Unexpected data structure:', data);
					driverSuggestions = [];
				}

				showSuggestions = driverSuggestions.length > 0;
				console.log('Processed suggestions:', driverSuggestions);
			} else {
				console.error('API response not OK:', response.status, response.statusText);
				driverSuggestions = [];
				showSuggestions = false;
			}
		} catch (error) {
			console.error('Error searching drivers:', error);
			driverSuggestions = [];
			showSuggestions = false;
		}
	}, 300);

	function selectDriver(driver: any) {
		formData.taxi_driver_name = driver.name;
		formData.taxi_license_plate = driver.license_plate;
		showSuggestions = false;
	}

	function handleDriverNameInput(event: Event) {
		const input = event.target as HTMLInputElement;
		formData.taxi_driver_name = input.value;
		searchDrivers(input.value);
	}

function searchLocations(query: string) {
    locationSearch = query;
    if (query.length < 1) {
        filteredLocations = [];
        showLocationSuggestions = false;
        return;
    }

    const lowercaseQuery = query.toLowerCase().trim();
    const queryParts = lowercaseQuery.split(/\s+/);

    filteredLocations = locations.filter((location) => {
        const lowercaseCountry = location.country.toLowerCase();
        const lowercaseState = location.state.toLowerCase();
        
        // Check if all parts of the query are included in either country or state
        return queryParts.every(part => 
            lowercaseCountry.includes(part) || 
            lowercaseState.includes(part) ||
            `${lowercaseCountry} ${lowercaseState}`.includes(part)
        );
    });

    showLocationSuggestions = filteredLocations.length > 0;
}

	function selectLocation(location: (typeof locations)[0]) {
		formData.location_id = location.id;
		locationSearch = `${location.country} - ${location.state}`;
		showLocationSuggestions = false;
	}

	async function handleSubmit() {
		if (
			!formData.taxi_driver_name ||
			!formData.taxi_license_plate ||
			!formData.location_id ||
			!formData.taxi_application ||
			!formData.description
		) {
			submitError = 'Por favor, complete todos los campos obligatorios.';
			return;
		}

		isSubmitting = true;
		submitError = '';

		try {
			// Upload driver image if exists
			if (driverImageFile) {
				const uploadUrl = await getImageUploadUrl('driver_images');
				await uploadImage(uploadUrl, driverImageFile);
				formData.driver_image = uploadUrl.publicUrl;
			}

			// Upload complaint images
			formData.complaint_images = await Promise.all(
				complaintImageFiles.map(async (file: any) => {
					const uploadUrl = await getImageUploadUrl('complaint_images');
					await uploadImage(uploadUrl, file);
					return uploadUrl.publicUrl;
				})
			);

			// Submit the complaint
			const response = await fetch('http://localhost:4200/api/taxi/complaint', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(formData)
			});

			if (!response.ok) {
				throw new Error('Failed to submit complaint');
			}

			submitSuccess = true;
      await goto('/');
		} catch (error) {
			submitError = 'Error submitting complaint. Please try again.';
			console.error('Error:', error);
		} finally {
			isSubmitting = false;
		}
	}

	async function getImageUploadUrl(prefix: string) {
		const response = await fetch('http://localhost:4200/api/taxi/generate-image-upload-url', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ prefix })
		});
		return await response.json();
	}

	async function uploadImage(uploadUrl: { url: string; publicUrl: string }, file: File) {
		await fetch(uploadUrl.url, {
			method: 'PUT',
			body: file
		});
	}

	function handleDriverImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		if (target.files) {
			driverImageFile = target.files[0];
		}
	}

	function handleComplaintImagesChange(event: Event) {
		const target = event.target as HTMLInputElement;
		if (target.files) {
			complaintImageFiles = Array.from(target.files);
		}
	}

	function removeDriverImage() {
		driverImageFile = null;
	}

	function removeComplaintImage(index: number) {
		complaintImageFiles = complaintImageFiles.filter((_, i) => i !== index);
	}

	function selectTaxiApp(appName: string) {
		formData.taxi_application = appName;
	}
</script>

{#if visible}
	<div
		class="min-h-screen bg-gray-900 px-4 py-12 text-white sm:px-6 lg:px-8"
		in:fade={{ duration: 300 }}
	>
		<div class="mx-auto max-w-4xl">
			<h1 class="mb-8 text-center text-4xl font-extrabold text-yellow-500">Reportar un Taxista</h1>

			{#if submitSuccess}
				<div
					class="mb-8 rounded-lg bg-green-600 p-6 text-white shadow-lg"
					in:fly={{ y: 20, duration: 300 }}
				>
					<div class="flex items-center">
						<CheckCircle class="mr-4 h-8 w-8" />
						<p class="text-lg font-semibold">
							Reporte enviado exitosamente. Gracias por ayudar a mantener segura nuestra comunidad.
						</p>
					</div>
				</div>
			{:else}
				<form
					on:submit|preventDefault={handleSubmit}
					class="space-y-8 rounded-lg bg-gray-800 p-8 shadow-xl"
				>
					<div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
						<div class="relative col-span-2">
							<label for="taxi_driver_name" class="mb-2 block text-sm font-medium text-gray-300"
								>Nombre del Taxista *</label
							>
							<input
								type="text"
								id="taxi_driver_name"
								bind:value={formData.taxi_driver_name}
								required
								on:input={handleDriverNameInput}
								autocomplete="off"
								class="mt-1 block w-full rounded-md border-gray-600 bg-gray-700 px-4 py-3 text-lg text-white shadow-sm transition duration-150 ease-in-out focus:border-yellow-500 focus:ring-yellow-500"
							/>
							{#if showSuggestions}
								<div
									class="absolute z-50 mt-1 max-h-60 w-full overflow-auto rounded-md bg-gray-800 shadow-lg"
								>
									{#each driverSuggestions as driver}
										<div
											on:click={() => selectDriver(driver)}
											class="flex cursor-pointer items-center border-b border-gray-700 px-4 py-2 last:border-b-0 hover:bg-gray-700"
										>
											<Search class="mr-2 h-4 w-4 text-yellow-500" />
											<span class="text-white">{driver.name} - {driver.license_plate}</span>
										</div>
									{/each}
								</div>
							{/if}
						</div>

						<div class="col-span-2">
							<label for="taxi_license_plate" class="mb-2 block text-sm font-medium text-gray-300"
								>Placa del Taxi *</label
							>
							<input
								type="text"
								id="taxi_license_plate"
								bind:value={formData.taxi_license_plate}
								required
								autocomplete="off"
								class="mt-1 block w-full rounded-md border-gray-600 bg-gray-700 px-4 py-3 text-lg text-white shadow-sm transition duration-150 ease-in-out focus:border-yellow-500 focus:ring-yellow-500"
							/>
						</div>
					</div>

					<div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
						<div class="relative col-span-2">
							<label for="location_search" class="mb-2 block text-sm font-medium text-gray-300"
								>Ubicación *</label
							>
							<input
								type="text"
								id="location_search"
								bind:value={locationSearch}
								on:input={() => searchLocations(locationSearch)}
								placeholder="Buscar por país o estado"
								required
								autocomplete="off"
								class="mt-1 block w-full rounded-md border-gray-600 bg-gray-700 px-4 py-3 text-lg text-white shadow-sm transition duration-150 ease-in-out focus:border-yellow-500 focus:ring-yellow-500"
							/>
							{#if showLocationSuggestions}
								<div
									class="absolute z-50 mt-1 max-h-60 w-full overflow-auto rounded-md bg-gray-800 shadow-lg"
								>
									{#each filteredLocations as location}
										<div
											on:click={() => selectLocation(location)}
											class="flex cursor-pointer items-center border-b border-gray-700 px-4 py-2 last:border-b-0 hover:bg-gray-700"
										>
											<Search class="mr-2 h-4 w-4 text-yellow-500" />
											<span class="text-white">{location.country} - {location.state}</span>
										</div>
									{/each}
								</div>
							{/if}
						</div>

						<div class="col-span-2">
							<label class="mb-2 block text-sm font-medium text-gray-300"
								>Aplicación de Taxi *</label
							>
							<div class="flex flex-wrap gap-4">
								{#each taxiApps as app}
									<button
										type="button"
										class="flex flex-col items-center rounded-lg bg-gray-700 p-2 transition-colors duration-200 hover:bg-gray-600 {formData.taxi_application ===
										app.name
											? 'ring-2 ring-yellow-500'
											: ''}"
										on:click={() => selectTaxiApp(app.name)}
									>
										<img src={app.icon} alt={app.name} class="mb-1 h-12 w-12" />
										<span class="text-xs">{app.name}</span>
									</button>
								{/each}
							</div>
						</div>
					</div>

					<div>
						<label for="description" class="mb-2 block text-sm font-medium text-gray-300"
							>Descripción del Incidente *</label
						>
						<textarea
							id="description"
							bind:value={formData.description}
							required
							rows="4"
							class="mt-1 block w-full rounded-md border-gray-600 bg-gray-700 px-4 py-3 text-lg text-white shadow-sm transition duration-150 ease-in-out focus:border-yellow-500 focus:ring-yellow-500"
						></textarea>
					</div>

					<div>
						<label class="mb-2 block text-sm font-medium text-gray-300">Imagen del Taxista</label>
						<div class="flex items-center space-x-4">
							<label
								for="driver_image"
								class="inline-flex cursor-pointer items-center rounded bg-yellow-500 px-4 py-2 font-bold text-white transition duration-150 ease-in-out hover:bg-yellow-600"
							>
								<Camera class="mr-2 h-4 w-4" />
								<span>Subir Imagen</span>
							</label>
							<input
								type="file"
								id="driver_image"
								accept="image/*"
								on:change={handleDriverImageChange}
								autocomplete="off"
								class="hidden"
							/>
							{#if driverImageFile}
								<div class="flex items-center rounded-md bg-gray-700 px-3 py-2">
									<span class="mr-2 text-sm text-gray-300">{driverImageFile.name}</span>
									<button
										type="button"
										on:click={removeDriverImage}
										class="text-red-400 hover:text-red-500"
									>
										<X class="h-4 w-4" />
									</button>
								</div>
							{/if}
						</div>
					</div>

					<div>
						<label class="mb-2 block text-sm font-medium text-gray-300">Imágenes de la Queja</label>
						<div class="mb-4 flex items-center space-x-4">
							<label
								for="complaint_images"
								class="inline-flex cursor-pointer items-center rounded bg-yellow-500 px-4 py-2 font-bold text-white transition duration-150 ease-in-out hover:bg-yellow-600"
							>
								<Upload class="mr-2 h-4 w-4" />
								<span>Subir Imágenes</span>
							</label>
							<input
								type="file"
								id="complaint_images"
								accept="image/*"
								autocomplete="off"
								multiple
								on:change={handleComplaintImagesChange}
								class="hidden"
							/>
						</div>
						{#if complaintImageFiles.length > 0}
							<div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
								{#each complaintImageFiles as file, index}
									<div class="relative rounded-md bg-gray-700 p-2">
										<span class="text-sm text-gray-300">{file.name}</span>
										<button
											type="button"
											on:click={() => removeComplaintImage(index)}
											class="absolute right-1 top-1 text-red-400 hover:text-red-500"
										>
											<X class="h-4 w-4" />
										</button>
									</div>
								{/each}
							</div>
						{/if}
					</div>

					{#if submitError}
						<div class="rounded-lg bg-red-600 p-4 text-white" in:fly={{ y: 20, duration: 300 }}>
							<div class="flex items-center">
								<AlertCircle class="mr-2 h-5 w-5" />
								<p>{submitError}</p>
							</div>
						</div>
					{/if}

					<button
						type="submit"
						disabled={isSubmitting || !formData.taxi_application}
						class="focus:shadow-outline w-full rounded-lg bg-yellow-500 px-6 py-4 text-lg font-bold text-white transition duration-150 ease-in-out hover:bg-yellow-600 focus:outline-none {!formData.taxi_application ||
						isSubmitting
							? 'cursor-not-allowed opacity-50'
							: ''}"
					>
						{isSubmitting ? 'Enviando...' : 'Enviar Reporte'}
					</button>
				</form>
			{/if}
		</div>
	</div>
{/if}
