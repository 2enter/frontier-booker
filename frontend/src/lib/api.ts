import type { Cargo, CargoRequest } from '@/types/model';

import axios from 'axios';
import { Api } from '@2enter/web-kit/runtime';
import { objToFD } from '@2enter/web-kit/calc';

const api = new Api(axios.create());

async function getNews() {
	return api.fetch<string[]>({ url: '/api/news' });
}

async function getSysTemp() {
	return api.fetch<number>({ url: '/api/sys-temp' });
}

async function postCargo(input: CargoRequest) {
	const fd = objToFD({ ...input });
	return api.fetch<Cargo>({ url: '/api/cargo', method: 'post', data: fd });
}

async function getCargoes() {
	return api.fetch<Cargo[]>({ url: '/api/cargo' });
}

async function getTodayCargoes() {
	return api.fetch<Cargo[]>({ url: '/api/cargo/today' });
}

async function getCargoById(id: string) {
	return api.fetch<Cargo>({ url: `/api/cargo/${id}` });
}

export { getCargoes, getNews, getSysTemp, getTodayCargoes, postCargo, getCargoById };
