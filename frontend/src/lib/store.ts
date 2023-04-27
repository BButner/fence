import { writable, type Writable } from 'svelte/store';
import { GrpcStatus } from './types/grpc-status';
import { invoke } from '@tauri-apps/api';
import type { IStateResponse } from './types/state';
import type { IDisplay } from './types/displays';
import type { IRegion } from './types/regions';

export class FenceClientStore {
	constructor(
		public hostname: Writable<string | undefined> = writable(undefined),
		public grpcStatus: Writable<GrpcStatus> = writable(GrpcStatus.DISCONNECTED),
		public displays: Writable<IDisplay[]> = writable([]),
		public regions: Writable<IRegion[]> = writable([])
	) {}

	public connectGrpc = async (hostname: string) => {
		invoke('connect_grpc', { hostname });
	};

	public refreshDisplays = async () => {
		console.log('refreshDisplays');
		await invoke<IDisplay[]>('get_displays').then((displays) => {
			console.log('get_displays', displays);
			this.displays.update(() => displays);
		});
	};

	public updateFromState = async () => {
		await invoke<IStateResponse>('get_state').then((state) => {
			this.hostname.set(state.currentHostname);
			this.grpcStatus.set(state.grpcStatus);
		});

		await invoke<IDisplay[]>('get_displays').then((displays) => {
			console.log('get_displays', displays);
			this.displays.update(() => displays);
		});

		await invoke<IRegion[]>('get_regions').then((regions) => {
			this.regions.update(() => regions);
		});
	};
}
