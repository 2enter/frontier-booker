const RESULT_TESTING = false;

const COLORS = [
	{ name: 'red', value: '#EB0018' },
	{ name: 'orange', value: '#ED6A19' },
	{ name: 'yellow', value: '#DFDD2C' },
	{ name: 'lime', value: '#84D044' },
	{ name: 'green', value: '#2E8250' },
	{ name: 'blue', value: '#3E67D6' },
	{ name: 'cyan', value: '#53C6D9' },
	{ name: 'purple', value: '#855AA8' },
	{ name: 'pink', value: '#EB7DD1' },
	{ name: 'peach', value: '#EDBEAA' },
	{ name: 'brown', value: '#723D1E' },
	{ name: 'black', value: '#262626' }
] as const;

type ColorName = (typeof COLORS)[number]['name'];

const DEFAULT_CRON_CONFIG = {
	timezone: 'Asia/Taipei',
	start: true,
	runOnInit: true
} as const;

export { COLORS, DEFAULT_CRON_CONFIG, RESULT_TESTING };

export type { ColorName };
