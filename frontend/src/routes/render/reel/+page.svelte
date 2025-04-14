<script lang="ts">
	import type { Cargo } from '@/types/model';
	import moment from 'moment';
	import { onMount } from 'svelte';
	import { getCargoes } from '@/api';
	import { page } from '$app/state';

	let cargoes = $state<Cargo[]>([]);
	const asMask = $derived(page.url.hash === '#mask');

	onMount(() => {
		console.log(page.url);
		const interval = setInterval(async () => {
			const { data, error } = await getCargoes();
			if (error) {
				console.error(error);
				return;
			}

			cargoes = data ?? [];
		}, 1000);

		return () => {
			clearInterval(interval);
		};
	});
</script>

<div class="center-content full-screen bg-black px-3 text-white">
	<div class="flex w-fit flex-row gap-2 overflow-x-hidden">
		{#each cargoes as { id, name, createdAt }}
			<div class="flex h-72 w-48 flex-col items-center">
				<span class:text-black={!asMask}>{moment(createdAt).format('YY/MM/DD HH:mm')}</span>
				<div class="size-48 bg-white">
					{#if !asMask}
						<img src="/api/storage/texture/{id}.jpg" class="size-48" alt="" />
					{/if}
				</div>
				<span class:text-black={!asMask} class="text-center text-xl">{name}</span>
			</div>
		{/each}
	</div>
</div>
