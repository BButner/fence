import type { GrpcStatus } from './grpc-status';

export interface IStateResponse {
	currentHostname?: string;
	grpcStatus: GrpcStatus;
}
