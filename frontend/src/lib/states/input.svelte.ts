import type { ParseEnum } from '@2enter/web-kit/types';
import type { Cargo } from '@/types/model';
import { CargoStatus, CargoType } from '@/types/model'

import { getContext, setContext } from 'svelte';
import axios from 'axios';

const _TEST_CARGO = {
	id: "07af5db7-1ccd-42e3-8882-6fac04c5fb3b",
	type: CargoType.Cake,
	createdAt: new Date(),
	status: CargoStatus.Delivered,
	paintTime: 30,
	name: "彩虹絲帶",
	description: "這是一種由特殊纖維編織而成的柔軟絲帶，在任何光線下都能發出柔和的變幻色彩。它不僅可以用來裝飾和包裝，還具有極強的韌性，能在太空環境中保持穩定性。這種絲帶的獨特之處在於它能根據周圍溫度改變顏色，是太空站最受歡迎的包裝材料之一。",
}

class InputState {
	cargoType = $state<ParseEnum<CargoType> | null>(null);
	drawDuration = $state(0);

	result = $state<Cargo | null>();

	resultImgUrl = $state<string | null>(null);

	readonly submittable = $derived(this.cargoType && this.drawDuration);
	readonly requestMetadata = $derived({ type: this.cargoType, paintTime: this.drawDuration });

	reset = () => {
		this.cargoType = null;
		this.drawDuration = 0;
		this.result = null;
		this.resultImgUrl = null;
	};

	async getPaint() {
		const url = this.resultImgUrl;
		if (!url) return null;
		return await axios.get<Blob>(url, { responseType: 'blob' }).then((res) => res.data);
		// return new File([blob], 'paint.png');
	}
}

const INPUT_STATE_CTX = 'INPUT_STATE';

function setInputState() {
	return setContext(INPUT_STATE_CTX, new InputState());
}

function getInputState() {
	return getContext<ReturnType<typeof setInputState>>(INPUT_STATE_CTX);
}

export { setInputState, getInputState };
