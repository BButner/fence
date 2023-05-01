import type { GrpcStatus } from "./grpc-status"

export interface IFenceState {
  currentHostname?: string
  grpcStatus: GrpcStatus
}
