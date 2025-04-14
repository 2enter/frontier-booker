<script lang="ts">
	import { getSysState } from '@/states';
	import { Pages } from './pages';
	import { fade } from 'svelte/transition';
	import { imageModules } from '@/assets/images';

	const sysState = getSysState();

	const Page = $derived(Pages[sysState.pageNum]);
</script>

{#key sysState.pageNum}
	<div in:fade class="center-content" class:mix-blend-multiply={sysState.pageNum !== 5}>
		<Page />
	</div>
{/key}

{#each Object.entries(imageModules) as [_, { default: { img: { src } } }]}
	<link rel="preload" as="image" href={src} />
{/each}
