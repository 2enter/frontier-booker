<script lang="ts">
	import * as THREE from 'three';
	import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
	import SuccessImage from '@/assets/ui/texts/upload_success.webp?enhanced';

	import { getSysState, getInputState } from '@/states';
	import { onDestroy, onMount } from 'svelte';

	import { ImgBtn } from '@/components';
	import { getCargoById } from '@/api';
	import { fade, scale } from 'svelte/transition';
	import { getImageSrc } from '@/assets/images';

	// declare constants
	const FRAME_RATE = 30;
	const ROTATE_RANDOMNESS = {
		x: Math.random() * 0.4 - 0.2,
		y: Math.random() * 0.1 - 0.05,
		z: Math.random() * 0.05 - 0.025
	};
	const scene = new THREE.Scene();
	const loader = new GLTFLoader();
	const textureLoader = new THREE.TextureLoader();

	// declare mutables
	let cargoModel: THREE.Object3D;
	let threeDom: HTMLDivElement;
	let frame = 0;
	let lastTouch: Touch | null = null;
	let nameDom = $state<HTMLHeadElement>();
	let pendingDotNum = $state(3);
	let showInfo = $state(false);

	// declare states
	const [inputState, sysState] = [getInputState(), getSysState()];
	const cargo = $derived(inputState.result);
	let textInfo = $state<{ name: string; description: string } | null>();

	$effect(() => {
		console.log(nameDom?.clientHeight);
	});

	onMount(async () => {
		if (!inputState.result) return;

		const { type, id } = $state.snapshot(inputState.result);

		if (cargo?.name && cargo?.description) {
			// For testing, skipping the remote data fetching
			textInfo = { name: cargo.name, description: cargo.description };
		} else {
			const pendingInterval = setInterval(() => (pendingDotNum = (pendingDotNum + 1) % 10), 500);
			// fetch cargo text info through backend API
			const interval = setInterval(async () => {
				const { data: updated } = await getCargoById(id);
				if (!updated || !updated.name || !updated.description) return;

				textInfo = { name: updated.name, description: updated.description };
				console.table(textInfo);
				setTimeout(() => {
					showInfo = true;
					clearInterval(interval);
					clearInterval(pendingInterval);
				});
			}, 1000);
		}

		// declare basic THREE objects
		const camera = new THREE.PerspectiveCamera(
			75,
			window.innerWidth / window.innerHeight,
			0.1,
			1000
		);
		const renderer = new THREE.WebGLRenderer({ alpha: true });
		const light = new THREE.PointLight('white', 80);

		// load texture
		const texture = await textureLoader.loadAsync(`/api/storage/texture/${id}.jpg`);
		texture.flipY = false;

		// make material from texture
		const material = new THREE.MeshToonMaterial({ map: texture });

		// make cargo model from material
		cargoModel = await loader
			.loadAsync(`/cargoes/${type}.glb`)
			.then(({ scene }) => scene.children[0]);
		cargoModel.rotation.y = 90;
		if ('material' in cargoModel) cargoModel.material = material;

		// THREE parameters initialization
		renderer.setSize(window.innerWidth, window.innerHeight);
		light.position.set(2, 2, 4.5);
		camera.position.z = 3.6;
		camera.position.y = 1.5;
		camera.lookAt(0, 0, 0);

		// append THREE to DOM
		if (threeDom) threeDom.appendChild(renderer.domElement);
		// threeCanvas = renderer.domElement;

		// add objects to scene
		scene.add(cargoModel);
		scene.add(light);

		function animate() {
			setTimeout(() => {
				frame = requestAnimationFrame(animate);
				renderer.render(scene, camera);
				for (const dir of ['x', 'y', 'x'] as const) {
					if (!lastTouch) cargoModel.rotation[dir] += ROTATE_RANDOMNESS[dir] / FRAME_RATE;
					cargoModel.rotation[dir] = cargoModel.rotation[dir] % (Math.PI * 2);
				}
			}, 1000 / FRAME_RATE);
		}
		animate();
	});

	function onTouchMove(e: TouchEvent) {
		const currentTouch = e.touches[0];
		if (lastTouch && cargoModel) {
			const touchDiff = [
				currentTouch.clientX - lastTouch.clientX,
				currentTouch.clientY - lastTouch.clientY
			];

			cargoModel.rotation.y += touchDiff[0] / 200;
			cargoModel.rotation.x += touchDiff[1] / 1000;
		}
		lastTouch = e.touches[0];
	}

	function onTouchEnd() {
		lastTouch = null;
	}

	onDestroy(() => {
		scene.clear();
		cancelAnimationFrame(frame);
	});
</script>

<div
	bind:this={threeDom}
	class="full-screen z-[1000]"
	hidden={showInfo}
	ontouchmove={onTouchMove}
	ontouchend={onTouchEnd}
></div>

{#if textInfo}
	{#if showInfo}
		{@const { name, description } = textInfo}
		{@const nameIsLong = (nameDom?.clientHeight ?? 0) > 48}
		<div transition:fade class="full-screen z-[1000] bg-white/30"></div>
		<ImgBtn
			src="/ui/buttons/close.png"
			class="fixed right-24 top-24 z-[2000] size-24"
			onclick={() => (showInfo = false)}
		/>
		<div
			transition:scale={{ duration: 369 }}
			class="font-dot-gothic reel fixed z-[1200] flex h-[70vh] w-[70vw] flex-col items-center justify-start gap-3 rounded-xl px-6 text-3xl text-black/80"
		>
			<h1 bind:this={nameDom} class="z-[3000] mt-20 max-w-[40vw] text-center text-5xl font-bold">
				{name}
			</h1>
			<img src={inputState.resultImgUrl} class="h-fit w-[39%]" alt="" />
			<p
				class="z-[3000] mr-6 w-[67%] rotate-[1deg] overflow-y-scroll rounded-xl leading-tight"
				style:height="{nameIsLong ? '20' : '26'}%"
				style:text-size="{nameIsLong ? '25' : '33'}px"
			>
				{description}
			</p>
		</div>
	{/if}
{:else}
	<div
		class="font-dot-gothic pointer-events-none fixed top-64 z-[1500] w-full bg-black/90 p-2.5 text-center text-4xl text-white"
	>
		{'·'.repeat(pendingDotNum)}圖鑑內文生成中{'·'.repeat(pendingDotNum)}
	</div>
{/if}

{#if !showInfo}
	<div class="full-screen pt-15 flex flex-col items-center justify-between px-12 pb-52 pt-10">
		<enhanced:img src={SuccessImage} alt="" />
		<div class="flex flex-col">
			{#each { length: 3 } as _, n}
				<enhanced:img src={getImageSrc(`/ui/texts/headup_${n}.png`)} class="h-12 w-auto" alt="" />
			{/each}
		</div>
	</div>
{/if}

{#if textInfo}
	<div class="center-content fixed bottom-20 z-[3000] flex w-screen gap-12 opacity-80 *:w-64">
		{#if !showInfo}
			<ImgBtn src="/ui/buttons/open_reel.png" onclick={() => (showInfo = true)} />
		{/if}
		<ImgBtn src="/ui/buttons/restart.png" onclick={() => window.location.reload()} />
	</div>
{/if}

<style>
	@import url('https://fonts.googleapis.com/css2?family=DotGothic16&display=swap');

	.reel {
		/* background-image: var(--result-url), url('@/assets/ui/layouts/reel.png'); */
		background-image: url('@/assets/ui/layouts/reel.png');
		background-position: center;
		background-size: contain;
		background-repeat: no-repeat;
	}
</style>
