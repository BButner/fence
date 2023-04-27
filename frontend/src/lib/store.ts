import { get, writable, type Writable } from 'svelte/store';
import { GrpcStatus } from './types/grpc-status';
import { invoke } from '@tauri-apps/api';
import type { IStateResponse } from './types/state';

export class FenceClientStore {
	constructor(
		public hostname: Writable<string | undefined> = writable(undefined),
		public grpcStatus: Writable<GrpcStatus> = writable(GrpcStatus.DISCONNECTED)
	) {}

	public connectGrpc = async (hostname: string) => {
		invoke('connect_grpc', { hostname });
	};

	public updateFromState = async () => {
		invoke<IStateResponse>('get_state').then((state) => {
			this.hostname.set(state.currentHostname);
			this.grpcStatus.set(state.grpcStatus);
		});
	};
}
