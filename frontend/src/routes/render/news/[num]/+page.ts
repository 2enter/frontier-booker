import type { EntryGenerator } from './$types';
import validator from 'validator';

export const load = async ({ params }) => {
	const { num } = params;

	if (!validator.isInt(num)) return { num: 0 };

	return {
		num: +num
	};
};

export const entries: EntryGenerator = () => {
	return [0, 1, 2, 3, 4].map((n) => {
		return { num: n.toString() };
	});
};
