export interface Request {
  id: string,
  status: RequestStatus,
  corkId?: string,
  invalidateScope?: string,
  relayRequestTxHash: string,
  gmpTxHash: string,
  targetTxHash: string,

  votingPower: string,
  currentHeight: string,
  scheduledHeight: string,
  corkResult: string,
}

export enum RequestStatus {
  "Initialized",
  "Broadcasting",
  "Awaiting Consensus",
  "Scheduled",
  "Awaiting Relay",
  "Relayed",
  "Success"
}