<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { LoaderPinwheel } from '@lucide/svelte';
	import { navigating } from '$app/state';
	import { dev } from '$app/environment';
	// import { FullscreenChecker } from '@2enter/web-kit/components';

	import DistanceImage from '@/assets/ui/launch_time/distance.webp?enhanced';
	import MinuteImage from '@/assets/ui/launch_time/minute.webp?enhanced';
	import SecondImage from '@/assets/ui/launch_time/second.webp?enhanced';
	import LogoImage from '@/assets/ui/texts/2enter.webp?enhanced';

	import { getSysState } from '@/states';
	import { getLaunchCountDown } from '@/time';
	import { getImageSrc } from '@/assets/images';

	const sysState = getSysState();

	let { children } = $props();
	let launchCountDown = $state({
		min: 0,
		sec: 0
	});

	onMount(async () => {
		setInterval(() => {
			detectSWUpdate();
		}, 20000);

		if (dev) {
			const eruda = (await import('eruda')).default;
			eruda.init();
		}

		const interval = setInterval(() => {
			launchCountDown = getLaunchCountDown();
		}, 1234);

		window.oncontextmenu = (e) => {
			e.preventDefault();
			e.stopPropagation();
			return false;
		};

		return {
			destroy() {
				clearInterval(interval);
			}
		};
	});

	async function detectSWUpdate() {
		const registration = await navigator.serviceWorker.ready;

		registration.addEventListener('updatefound', () => {
			const newSW = registration.installing;
			newSW?.addEventListener('statechange', () => {
				if (newSW.state === 'installed') {
					if (confirm('New update available! Reload to update?')) {
						newSW.postMessage({ type: 'SKIP_WAITING' });
						window.location.reload();
					}
				}
			});
		});
	}
</script>

<div class="center-content fixed top-1 z-[1000] w-full px-40 *:h-8">
	{#if launchCountDown}
		<enhanced:img src={DistanceImage} alt="" />
		{#each ['min', 'sec'] as const as unit}
			{@const UnitImage = unit === 'min' ? MinuteImage : SecondImage}
			{#each launchCountDown[unit].toString() as digit}
				<enhanced:img
					class="py-2"
					src={getImageSrc(`/ui/launch_time/numbers/${digit}.webp`)}
					alt=""
				/>
			{/each}
			<enhanced:img src={UnitImage} alt="" />
		{/each}
	{/if}
</div>

<div class="center-content pointer-events-none fixed bottom-3 z-[1000] w-full">
	<enhanced:img src={LogoImage} class="h-16 w-auto" alt="" />
</div>

<div class="full-screen center-content bg-cover bg-center bg-no-repeat bg-{sysState.bgNum}">
	{@render children()}
</div>

<!-- <FullscreenChecker /> -->

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
		onclick={() => window.location.reload()}
	>
		reload
	</button>
{/if}

<style>
	.bg-0 {
		background-image: url('$lib/assets/ui/layouts/paper.webp');
	}
	.bg-1 {
		background-image: url('$lib/assets/ui/layouts/factory_bg.webp');
	}
</style>
