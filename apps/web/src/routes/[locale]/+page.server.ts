import type { Action, Actions } from '@sveltejs/kit';
import { fail } from '@sveltejs/kit';
import { CargoesTypeOptions, PBFile } from '@repo/lib/pb';
import { pb, ws } from '@/server';
import { makeTextureImage } from '@/image';

const submit: Action = async ({ request, fetch, url }) => {
	const formData = await request.formData();
	const draw_duration = +(formData.get('draw_duration') as string);
	const type = formData.get('cargo_type') as CargoesTypeOptions;
	const rawPaint = formData.get('paint') as string;

	const blob = await fetch(rawPaint).then((data) => data.blob());
	const buffer = await fetch(rawPaint).then((data) => data.arrayBuffer());

	const paint = new File([blob], 'paint.png');
	const texture = await makeTextureImage(buffer, type);

	// return fail(500, { message: 'failed' });

	const result = await pb
		.collection('cargoes')
		.create({
			draw_duration,
			type,
			paint,
			texture,
			status: 'shipping'
		})
		.catch((e) => {
			console.error(e);
			return null;
		});

	if (!result) return fail(500, { message: 'fail to upload' });

	ws.broadcast({
		data: {
			type: 'cargo',
			id: result.id,
			cargo_type: type,
			directory: `${url.origin}/api/texture/${result.id}`
		}
	});
	return result;
};

export const actions: Actions = { submit };
