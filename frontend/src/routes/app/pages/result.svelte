<script lang="ts">
	import * as THREE from 'three';
	import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

	import { getSysState, getInputState } from '@/states';
	import { onDestroy, onMount } from 'svelte';

	import { ImgBtn } from '@2enter/web-kit/components';
	import { getCargoById } from '@/api';

	const [inputState, sysState] = [getInputState(), getSysState()];

	const cargo = $derived(inputState.result);
	const loader = new GLTFLoader();
	const textureLoader = new THREE.TextureLoader();
	const scene = new THREE.Scene();

	let cargoModel: THREE.Object3D;
	let threeDom = $state<HTMLDivElement>();
	let frame = 0;
	let textInfo = $state<{ name: string; description: string } | null>();
	const FRAME_RATE = 30;

	onMount(async () => {
		if (!inputState.result) {
			console.log('result not found');
			return;
		}

		const { type, id } = $state.snapshot(inputState.result);

		if (cargo?.name && cargo?.description) {
			textInfo = { name: cargo.name, description: cargo.description };
		} else {
			// fetch cargo text info
			const interval = setInterval(async () => {
				const { data: updated } = await getCargoById(id);
				if (!updated || !updated.name || !updated.description) return;
				textInfo = { name: updated.name, description: updated.description };
				console.table(textInfo);
				inputState.reset();
				clearInterval(interval);
			}, 5000);
		}

		const model = await loader.loadAsync(`/cargoes/${type}.glb`);
		const texture = await textureLoader.loadAsync(`/api/storage/texture/${id}.jpg`);

		const camera = new THREE.PerspectiveCamera(
			75,
			window.innerWidth / window.innerHeight,
			0.1,
			1000
		);
		const renderer = new THREE.WebGLRenderer({ alpha: true });

		renderer.setSize(window.innerWidth, window.innerHeight);
		const light = new THREE.PointLight('white', 80);
		light.position.set(2, 2, 4.5);
		scene.add(light);

		if (threeDom) threeDom.appendChild(renderer.domElement);

		texture.flipY = false;

		const material = new THREE.MeshToonMaterial({ map: texture });
		cargoModel = model.scene.children[0];

		if ('material' in cargoModel) cargoModel.material = material;

		scene.add(cargoModel);

		camera.position.z = 3.6;
		camera.position.y = 1.5;

		camera.lookAt(0, 0, 0);

		cargoModel.rotation.y = 90;

		const randomness = {
			x: Math.random() * 0.4 - 0.2,
			y: Math.random() * 0.2 - 0.1,
			z: Math.random() * 0.05 - 0.025
		};

		function animate() {
			setTimeout(() => {
				frame = requestAnimationFrame(animate);
				renderer.render(scene, camera);
				for (const dir of ['x', 'y', 'x'] as const) {
					if (!lastTouch) cargoModel.rotation[dir] += randomness[dir] / FRAME_RATE;
					// cargoModel.rotation[dir] = cargoModel.rotation[dir] % (Math.PI * 2);
				}
			}, 1000 / FRAME_RATE);
		}
		animate();
	});

	let lastTouch: Touch | null = null;

	function onTouchMove(e: TouchEvent) {
		const currentTouch = e.touches[0];
		if (lastTouch && cargoModel) {
			const touchDiff = [
				currentTouch.clientX - lastTouch.clientX,
				currentTouch.clientY - lastTouch.clientY
			] as const;

			// do something
			cargoModel.rotation.y += touchDiff[0] / 200;
			cargoModel.rotation.x += touchDiff[1] / 1000;
			// cargoModel.rotation.z += touchDiff[0] / 10000;
			console.log(touchDiff);
		}
		lastTouch = e.touches[0];
	}

	onDestroy(() => {
		scene.clear();
		cancelAnimationFrame(frame);
	});
</script>

<div
	bind:this={threeDom}
	class="full-screen z-[1000]"
	ontouchmove={onTouchMove}
	ontouchend={() => (lastTouch = null)}
></div>

<div
	class="font-dot-gothic absolute top-40 z-[1200] flex h-fit w-[90%] flex-col gap-2 rounded-xl p-3 text-center text-3xl text-black"
>
	{#if textInfo}
		{@const { name, description } = textInfo}
		<h1 class="text-4xl font-bold">『{name}』</h1>
		<p class="text-[21px] leading-tight">{description}</p>
	{:else}
		pending
	{/if}
</div>

<div class="full-screen pt-15 flex flex-col justify-between px-12 pb-40">
	<img src="/ui/texts/upload_success.webp" alt="" />
	<img src="/ui/texts/head_up.webp" alt="" />
</div>

{#if textInfo}
	<ImgBtn
		class="fixed bottom-12 z-[3000] w-56"
		src="/ui/buttons/restart.webp"
		onclick={() => {
			inputState.reset();
			sysState.routeTo(0);
		}}
	/>
{/if}
