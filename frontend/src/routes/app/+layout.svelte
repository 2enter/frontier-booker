<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { LoaderPinwheel } from '@lucide/svelte';
	import { navigating } from '$app/state';
	import { dev } from '$app/environment';
	import { FullscreenChecker } from '@2enter/web-kit/components';

	import { getSysState } from '@/states';
	import { getLaunchCountDown } from '@/time';

	const sysState = getSysState();

	let { children } = $props();
	let launchCountDown = $state({
		min: 0,
		sec: 0
	});

	onMount(async () => {
		if (dev) {
			const eruda = (await import('eruda')).default;
			eruda.init();
		}

		const interval = setInterval(() => {
			launchCountDown = getLaunchCountDown();
		}, 1234);

		return {
			destroy() {
				clearInterval(interval);
			}
		};
	});
</script>

<div class="center-content fixed top-1 z-[1000] w-full *:h-8">
	{#if launchCountDown}
		<img src="/ui/launch_time/distance.webp" alt="" />
		{#each launchCountDown.min.toString() as digit}
			<img class="py-2" src="/ui/launch_time/numbers/{digit}.webp" alt="" />
		{/each}
		<img src="/ui/launch_time/minute.webp" alt="" />
		{#each launchCountDown.sec.toString() as digit}
			<img class="py-2" src="/ui/launch_time/numbers/{digit}.webp" alt="" />
		{/each}
		<img src="/ui/launch_time/second.webp" alt="" />
	{/if}
</div>

<div class="center-content fixed bottom-3 z-[1000] w-full">
	<img src="/ui/texts/2enter.webp" class="h-10" alt="" />
</div>

<div
	class="full-screen center-content bg-cover bg-center bg-no-repeat"
	style:background-image="url({sysState.bg})"
>
	{@render children()}
</div>

<FullscreenChecker />

<dialog bind:this={sysState.dialog} class="modal modal-middle">
	<div class="modal-box">
		<h1>Error</h1>
		<p>{sysState.errorMessage}</p>
		<div class="modal-action">
			<form method="dialog">
				<button class="btn btn-secondary" onclick={sysState.closeError}> Close </button>
			</form>
		</div>
	</div>
</dialog>

{#if sysState.processing || navigating.to}
	<div
		transition:fade
		class="full-screen center-content z-[10000] bg-black/10 text-black backdrop-blur-sm"
	>
		<LoaderPinwheel size="120" strokeWidth="1" class="animate-spin overflow-hidden" />
	</div>
{/if}

{#if dev}
	<button
		class="btn btn-secondary fixed left-0 top-0 z-[3000]"
		onclick={() => window.location.reload()}>reload</button
	>
{/if}
