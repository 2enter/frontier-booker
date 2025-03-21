<script lang="ts">
	import '../app.css';
	import { setSysState, setInputState } from '@/states';

	setSysState();
	setInputState();

	let { children } = $props();
</script>

{@render children()}
