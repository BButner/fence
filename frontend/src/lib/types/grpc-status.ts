export enum GrpcStatus {
	CONNECTING = 'CONNECTING',
	CONNECTED = 'CONNECTED',
	DISCONNECTED = 'DISCONNECTED',
	ERROR = 'ERROR',
	CONNECTION_LOST = 'CONNECTION_LOST'
}

export interface IEventPayload {
	event: GrpcStatus;
	payload: string;
}
