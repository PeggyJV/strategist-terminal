export interface RequestState {
  id: string,
  request: ScheduleRequest,
  status: RequestStatus,
  corkId?: string,
  invalidateScope?: string,
  relayRequestTxHash: string,
  gmpTxHash: string,
  targetTxHash: string,
}

export enum ActiveRequestStatus {
  Initialized = "Initialized",
  Broadcasting = "Broadcasting",
  AwaitingVote = "AwaitingVote",
  AwaitingConfirmation = "AwaitingConfirmation",
  AwaitingRelay = "AwaitingRelay",
  Relayed = "Relayed",
  AwaitingExecution = "AwaitingExecution",
  Success = "Success",
}

export enum FailedRequestStatus {
  FailedBroadcast = "FailedBroadcast",
  FailedVote = "FailedVote",
  FailedExecution = "FailedExecution",
  Unknown = "Unknown"
}

export type RequestStatus = ActiveRequestStatus | FailedRequestStatus;


export interface ScheduleRequest {
    cellar_id: string,
    block_height: number,
    chain_id: number,
    deadline: number,
    call_data: any,
}
export enum Functions {
  CallOnAdaptor = "CallOnAdaptor",
  SetSharePriceOracle = "SetSharePriceOracle",
  AddPosition = "AddPosition"
}

export interface Adaptor {
  name: string
  address: string
  calls: AdaptorCall[]
}

export interface AdaptorCall {
  function: string
  action: string
  fields: Field[]

}
export interface Field {
  name: string
  label: string
  placeholder: string

}

export interface CellarCallInputs {
  function: Functions
  action: string
  fields: Field[]
}