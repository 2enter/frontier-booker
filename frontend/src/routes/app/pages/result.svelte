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
		const cargoModel = model.scene.children[0];

		if ('material' in cargoModel) cargoModel.material = material;

		scene.add(cargoModel);

		camera.position.z = 3.6;
		camera.position.y = 1.5;
		camera.lookAt(0, 0, 0);

		function animate() {
			setTimeout(() => {
				frame = requestAnimationFrame(animate);
				renderer.render(scene, camera);
				cargoModel.rotation.y += 1 / FRAME_RATE;
			}, 1000 / FRAME_RATE);
		}
		animate();
	});

	onDestroy(() => {
		scene.clear();
		cancelAnimationFrame(frame);
	});
</script>

<div bind:this={threeDom} class="full-screen z-[1000]"></div>

<div
	class="font-dot-gothic absolute top-40 z-[1200] flex h-fit w-[90%] flex-col gap-2 rounded-xl p-3 text-center text-3xl text-black"
>
	{#if textInfo}
		{@const { name, description } = textInfo}
		<h1 class="text-4xl font-bold">『{name}』</h1>
		<p>{description}</p>
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
