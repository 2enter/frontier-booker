<script lang="ts">
	import { ImgBtn } from '@/components';
	import { getSysState } from '@/states';

	const sysState = getSysState();
</script>

<div class="full-screen bg bg-cover bg-center"></div>

<ImgBtn
	src="/ui/buttons/start.webp"
	class="fixed bottom-[30vh] w-1/2"
	onclick={sysState.navigate}
/>

<style>
	.bg {
		background-image: url('@/assets/ui/layouts/intro_page.webp');
	}
</style>
