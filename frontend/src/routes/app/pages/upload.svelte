<script lang="ts">
	import type { CargoInput } from '@/types/model';
	import LightBeamImage from '@/assets/ui/animations/light_beam.webp';

	import { ImgBtn } from '@/components';
	import { makeSubmit } from '@2enter/web-kit/browser';
	import { getInputState, getSysState } from '@/states';
	import { postCargo } from '@/api';

	const [inputState, sysState] = [getInputState(), getSysState()];

	const lightBeam = (node: HTMLImageElement) => {
		let num = 2;
		const interval = setInterval(() => {
			num += num;
			node.style.marginBottom = `${num}px`;
			if (num > window.innerHeight * 1.5) {
				sysState.navigate();
			}
		}, 100);

		return {
			destroy() {
				clearInterval(interval);
			}
		};
	};

	const submit = makeSubmit({
		state: sysState,
		process: async () => {
			if (!inputState.submittable) return '資料不足';

			const file = await inputState.getPaint();
			if (!file) return '圖片上傳失敗';

			// extract metadata from `inputState`
			const { type: cargoType, paintTime } = inputState.requestMetadata as CargoInput;
			const input = { paintTime, cargoType, file };

			// upload cargo image
			const { data: result, error } = await postCargo(input);
			console.error(error);

			if (!result) return '資料處理錯誤';

			// assign result to state
			inputState.result = result;
		}
	});
</script>

<link rel="preload" as="image" href={LightBeamImage} />

{#if !inputState.result}
	<img src={inputState.resultImgUrl} alt="" class="pointer-events-none fixed h-auto w-full" />
	{#if !sysState.processing}
		<ImgBtn src="/ui/buttons/upload.webp" class="z-[2000] w-2/3" onclick={submit} />
	{/if}
{:else}
	<img use:lightBeam class="h-[50vh] w-screen" src={LightBeamImage} alt="" />
{/if}
