<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { getSysState } from '@/states';
	import { getImageSrc } from '@/assets/images';

	const sysState = getSysState();

	let line = $state(0);

	onMount(() => {
		const interval = setInterval(() => {
			if (line === 8) {
				clearInterval(interval);
				return;
			}
			line++;
			console.log(line);
		}, 1500);

		return () => {
			clearInterval(interval);
		};
	});
</script>

{#each { length: line + 1 } as _, i}
	<img
		in:fade
		src={getImageSrc(`/ui/texts/description_page/${i}.png`).img.src}
		class="fixed left-4 top-0 h-screen w-[90%]"
		alt={i.toString()}
	/>
{/each}

<button
	class="full-screen z-[2000]"
	aria-label="button"
	onclick={() => {
		if (line < 7) {
			line = 7;
			return;
		}
		sysState.navigate();
	}}
></button>
