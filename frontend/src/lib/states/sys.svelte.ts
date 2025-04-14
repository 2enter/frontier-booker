import { setContext, getContext } from 'svelte';

class SysState {
	processing = $state(false);
	errorMessage = $state<string | null>(null);
	pageNum = $state<PageNum>(0);
	dialog = $state<HTMLDialogElement>();

	bgNum = $derived.by(() => {
		switch (this.pageNum) {
			case 0:
			case 3:
				return 0;
			default:
				return 1;
		}
	});

	popError = (message: string) => {
		this.errorMessage = message;
		if (!this.dialog) return;
		this.dialog.showModal();
	};

	startProcess = () => {
		this.processing = true;
	};

	endProcess = () => {
		this.processing = false;
	};

	closeError = () => {
		this.errorMessage = null;
		if (!this.dialog) return;
		this.dialog.close();
	};

	navigate = (step?: any) => {
		if (typeof step !== 'number') {
			step = 1;
		}

		let result = this.pageNum + step;

		if (result < 0 || result > 6) {
			console.error('invalid page navigation');
			return;
		}

		this.pageNum = result as PageNum;
	};

	routeTo = (num: PageNum) => {
		this.pageNum = num;
	};
}

const SYS_STATE_CTX = 'SYS_STATE';

function setSysState() {
	return setContext(SYS_STATE_CTX, new SysState());
}

function getSysState() {
	return getContext<ReturnType<typeof setSysState>>(SYS_STATE_CTX);
}

export { setSysState, getSysState };
