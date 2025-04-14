<script lang="ts">
	import { CargoType } from '@/types/model';
	import SelectCargoImage from '@/assets/ui/texts/select_cargo.webp?enhanced';

	import { getInputState, getSysState } from '@/states';
	import { ImgBtn } from '@/components';

	const [inputState, sysState] = [getInputState(), getSysState()];
	const CARGO_TYPES = Object.values(CargoType);

	function choose(type: CargoType) {
		inputState.cargoType = type;
		sysState.navigate();
	}
</script>

<div class="center-content flex-wrap gap-3 px-14">
	<enhanced:img src={SelectCargoImage} alt="" />
	{#each CARGO_TYPES as type}
		<ImgBtn src="/cargoes/{type}_select.webp" class="w-2/5" onclick={() => choose(type)}></ImgBtn>
	{/each}
</div>
