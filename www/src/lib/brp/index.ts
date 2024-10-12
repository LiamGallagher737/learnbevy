import type {
    BrpRequest,
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
    BrpResponse,
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

export async function brpRequest(request: GetRequest): Promise<GetResponse>;
export async function brpRequest(request: QueryRequest): Promise<QueryResponse>;
export async function brpRequest(request: SpawnRequest): Promise<SpawnResponse>;
export async function brpRequest(request: DestroyRequest): Promise<DestroyResponse>;
export async function brpRequest(request: RemoveRequest): Promise<RemoveResponse>;
export async function brpRequest(request: InsertRequest): Promise<InsertResponse>;
export async function brpRequest(request: ReparentRequest): Promise<ReparentResponse>;
export async function brpRequest(request: ListRequest): Promise<ListResponse>;
export async function brpRequest(request: GetWatchRequest): Promise<GetWatchResponse>;
export async function brpRequest(request: ListWatchRequest): Promise<ListWatchResponse>;

export async function brpRequest(
    req: BrpRequest,
): Promise<BrpResponse> {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const request: BrpRequest = {
        ...req,
        jsonrpc: req.jsonrpc ?? "2.0",
    };

    return {
        jsonrpc: "2.0",
        id: null,
        error: { code: 0, message: "Not implmented" },
    };
}
