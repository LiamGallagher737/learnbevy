import type {
    DestroyRequest,
    GetRequest,
    GetWatchRequest,
    InsertRequest,
    ListRequest,
    ListWatchRequest,
    QueryRequest,
    RemoveRequest,
    ReparentRequest,
    SpawnRequest,
} from "./request";
import type {
    DestroyResponse,
    GetResponse,
    GetWatchResponse,
    InsertResponse,
    ListResponse,
    ListWatchResponse,
    QueryResponse,
    RemoveResponse,
    ReparentResponse,
    SpawnResponse,
} from "./response";

export type BrpRequestFunc =
    ((request: GetRequest) => Promise<GetResponse>)
    | ((request: QueryRequest) => Promise<QueryResponse>)
    | ((request: SpawnRequest) => Promise<SpawnResponse>)
    | ((request: DestroyRequest) => Promise<DestroyResponse>)
    | ((request: RemoveRequest) => Promise<RemoveResponse>)
    | ((request: InsertRequest) => Promise<InsertResponse>)
    | ((request: ReparentRequest) => Promise<ReparentResponse>)
    | ((request: ListRequest) => Promise<ListResponse>)
    | ((request: GetWatchRequest) => Promise<GetWatchResponse>)
    | ((request: ListWatchRequest) => Promise<ListWatchResponse>);
