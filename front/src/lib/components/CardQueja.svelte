<script lang="ts">
	export let app: { name: string; logo: string };
	export let complaint: string;
	export let date: string;
</script>

<div
	class="rounded-xl bg-slate-700 bg-opacity-50 p-4 shadow-md backdrop-blur-sm transition-all hover:bg-opacity-70"
>
	<div class="mb-3 flex items-center">
		<div class="mr-3 h-10 w-10 rounded-full bg-slate-600 p-1.5 shadow-inner">
			<img src={app.logo} alt={app.name} class="h-full w-full object-contain" />
		</div>
		<div>
			<h4 class="font-semibold text-white">{app.name}</h4>
			<p class="text-sm text-slate-300">{date}</p>
		</div>
	</div>
	<p class="text-slate-200">{complaint}</p>
</div>
