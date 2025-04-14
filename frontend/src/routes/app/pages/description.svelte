<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { getSysState } from '@/states';
	import { getImageSrc } from '@/assets/images';

	const sysState = getSysState();

	let line = $state(0);

	onMount(() => {
		const interval = setInterval(() => {
			if (line === 7) {
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
	<enhanced:img
		in:fade
		src={getImageSrc(`/ui/texts/description_page/${i}.webp`)}
		class="full-screen m-auto"
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
