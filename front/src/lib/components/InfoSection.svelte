<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { onMount } from 'svelte';
	import { Shield, UserCheck, AlertTriangle } from 'lucide-svelte';

	let visible = false;

	onMount(() => {
		visible = true;
	});

	const infoItems = [
		{
			icon: Shield,
			title: 'Seguridad Primero',
			description:
				'Nuestra prioridad es tu seguridad. Verifica la información de tu taxista antes de subir al vehículo.'
		},
		{
			icon: UserCheck,
			title: 'Conductores Verificados',
			description:
				'Accede a información sobre conductores registrados y sus historiales de servicio.'
		},
		{
			icon: AlertTriangle,
			title: 'Reporta Incidentes',
			description:
				'Contribuye a la seguridad de la comunidad reportando cualquier incidente o comportamiento sospechoso.'
		}
	];
</script>

{#if visible}
	<section
		id="info-section"
		class="relative px-4 py-16 sm:px-6 sm:py-20 md:py-24 lg:py-32"
		in:fade={{ duration: 300, delay: 300 }}
	>
		<div class="container relative z-10 mx-auto">
			<h2
				class="mb-8 text-center text-3xl font-bold text-white sm:text-4xl md:mb-12 lg:mb-16 lg:text-5xl"
			>
				¿Por qué Checa Tu Taxi?
			</h2>
			<div class="grid gap-6 sm:gap-8 md:grid-cols-2 lg:grid-cols-3">
				{#each infoItems as { icon, title, description }, index}
					<div
						class="transform rounded-lg bg-slate-700 p-6 shadow-xl transition-transform duration-300 hover:scale-105 sm:p-8"
						in:fly={{ y: 20, duration: 400, delay: 300 + index * 100 }}
					>
						<svelte:component
							this={icon}
							class="mx-auto mb-4 h-12 w-12 text-yellow-400 sm:mb-6 sm:h-16 sm:w-16"
						/>
						<h3 class="mb-2 text-xl font-semibold text-white sm:mb-4 sm:text-2xl">{title}</h3>
						<p class="text-sm text-slate-300 sm:text-base">
							{description}
						</p>
					</div>
				{/each}
			</div>
		</div>
	</section>
{/if}
