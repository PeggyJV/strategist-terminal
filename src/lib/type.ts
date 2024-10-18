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
  AddPosition = "AddPosition",
  RemovePosition = "RemovePosition",
  SetHoldingPosition = "SetHoldingPosition",
  SetStrategistPayoutAddress = "SetStrategistPayoutAddress",
  SwapPositions = "SwapPositions",
  SetShareLockPeriod = "SetShareLockPeriod",
  InitiateShutdown = "InitiateShutdown",
  LiftShutdown = "LiftShutdown",
  RemoveAdaptorFromCatalogue = "RemoveAdaptorFromCatalogue",
  RemovePositionFromCatalogue = "RemovePositionFromCatalogue",
  DecreaseShareSupplyCap = "DecreaseShareSupplyCap",
  SetAlternativeAssetData = "SetAlternativeAssetData",
  DropAlternativeAssetData = "DropAlternativeAssetData",
  AddAdaptorToCatalogue = "AddAdaptorToCatalogue",
  AddPositionToCatalogue = "AddPositionToCatalogue",
  SetRebalanceDeviation = "SetRebalanceDeviation",
  SetStrategistPlatformCut = "SetStrategistPlatformCut",
  SetSharePriceOracle = "SetSharePriceOracle",
  IncreaseShareSupplyCap = "IncreaseShareSupplyCap",
  CachePriceRouter = "CachePriceRouter"
}
export enum FlashLoan {
  AaveV3DebtTokenV1FlashLoan = "AaveV3DebtTokenV1FlashLoan",
  BalancerPoolV1FlashLoan = "BalancerPoolV1FlashLoan"
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
  placeholder: PlaceHolder | string
  type?: "number" | "checkbox" | "text" | "array"

}

export interface CellarFunction {
  function: Functions
  action: string
  info?: string
  fields: Field[]
}

export enum PlaceHolder {
  Text = "e.g 1",
  Address = "e.g 0x1111111111111111111111111111111111111111",
  ArrayOfString = 'e.g ["1","2","3"]',
  ArrayOfAddress = 'e.g., ["0x000...", "0x000..."]',
  ArrayOfNumber = "e.g [1,2,3]",
  Empty = ""
}