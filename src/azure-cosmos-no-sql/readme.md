cargo run rusty-dina lists

how do I specify partition key

help me read documentation - https://docs.rs/azure_data_cosmos/latest/azure_data_cosmos/prelude/struct.CreateDocumentBuilder.html

```
vscode ➜ /workspaces/todo-rust-mongo-aca/src/azure-cosmos-no-sql (main) $ cargo run
   Compiling lock_api v0.4.11
   Compiling parking_lot_core v0.9.9
   Compiling smallvec v1.13.1
   Compiling scopeguard v1.2.0
   Compiling tokio-macros v2.2.0
   Compiling signal-hook-registry v1.4.1
   Compiling num_cpus v1.16.0
   Compiling parking_lot v0.12.1
   Compiling tokio v1.36.0
   Compiling tokio-util v0.7.10
   Compiling tokio-native-tls v0.3.1
   Compiling h2 v0.3.24
   Compiling hyper v0.14.28
   Compiling hyper-tls v0.5.0
   Compiling reqwest v0.11.24
   Compiling azure_core v0.19.0
   Compiling azure_data_cosmos v0.19.0
   Compiling azure-cosmos-no-sql v0.1.0 (/workspaces/todo-rust-mongo-aca/src/azure-cosmos-no-sql)
warning: unused import: `azure_core::Context`
 --> src/main.rs:6:5
  |
6 | use azure_core::Context;
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `azure-cosmos-no-sql` (bin "azure-cosmos-no-sql") generated 1 warning (run `cargo fix --bin "azure-cosmos-no-sql"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 18.08s
     Running `target/debug/azure-cosmos-no-sql`
thread 'main' panicked at src/main.rs:39:10:
please specify the database name as the first command line parameter
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
vscode ➜ /workspaces/todo-rust-mongo-aca/src/azure-cosmos-no-sql (main) $ cargo run rusty-dina lists
   Compiling parking_lot_core v0.9.9
   Compiling lock_api v0.4.11
   Compiling parking_lot v0.12.1
   Compiling tokio v1.36.0
   Compiling tokio-util v0.7.10
   Compiling tokio-native-tls v0.3.1
   Compiling h2 v0.3.24
   Compiling hyper v0.14.28
   Compiling hyper-tls v0.5.0
   Compiling reqwest v0.11.24
   Compiling azure_core v0.19.0
   Compiling azure_data_cosmos v0.19.0
^C  Building [=======================> ] 181/183: azure_data_cosmos                                                                                           
vscode ➜ /workspaces/todo-rust-mongo-aca/src/azure-cosmos-no-sql (main) $ ^C
vscode ➜ /workspaces/todo-rust-mongo-aca/src/azure-cosmos-no-sql (main) $ cargo run rusty-dina lists
   Compiling azure_data_cosmos v0.19.0
   Compiling azure-cosmos-no-sql v0.1.0 (/workspaces/todo-rust-mongo-aca/src/azure-cosmos-no-sql)
warning: unused import: `azure_core::Context`
 --> src/main.rs:6:5
  |
6 | use azure_core::Context;
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `azure-cosmos-no-sql` (bin "azure-cosmos-no-sql") generated 1 warning (run `cargo fix --bin "azure-cosmos-no-sql"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 8.29s
     Running `target/debug/azure-cosmos-no-sql rusty-dina lists`
Inserting 10 documents...
Error: Error { context: Full(Custom { kind: HttpResponse { status: BadRequest, error_code: Some("BadRequest") }, error: HttpError { status: BadRequest, details: ErrorDetails { code: Some("BadRequest"), message: Some("Message: {\"Errors\":[\"PartitionKey extracted from document doesn't match the one specified in the header. Learn more: https:\\/\\/aka.ms\\/CosmosDB\\/sql\\/errors\\/wrong-pk-value\"]}\r\nActivityId: 2757e3d0-24f7-4465-b3ec-ce6264f6c0fe, Request URI: /apps/56ada6bd-5a60-4436-9835-23212828207c/services/f1d5c6d3-dc85-49e1-9ef8-bc161d47593a/partitions/845f30bb-1026-45d3-96c1-de139fb9e9d6/replicas/133526739009068699p/, RequestStats: \r\nRequestStartTime: 2024-02-17T23:02:08.1172474Z, RequestEndTime: 2024-02-17T23:02:08.1391649Z,  Number of regions attempted:1\r\n{\"systemHistory\":[{\"dateUtc\":\"2024-02-17T23:01:10.7337246Z\",\"cpu\":0.302,\"memory\":409273560.000,\"threadInfo\":{\"isThreadStarving\":\"False\",\"threadWaitIntervalInMs\":0.0466,\"availableThreads\":32760,\"minThreads\":64,\"maxThreads\":32767},\"numberOfOpenTcpConnection\":517},{\"dateUtc\":\"2024-02-17T23:01:20.7436998Z\",\"cpu\":0.539,\"memory\":409272336.000,\"threadInfo\":{\"isThreadStarving\":\"False\",\"threadWaitIntervalInMs\":0.0306,\"availableThreads\":32764,\"minThreads\":64,\"maxThreads\":32767},\"numberOfOpenTcpConnection\":517},{\"dateUtc\":\"2024-02-17T23:01:30.7537134Z\",\"cpu\":0.805,\"memory\":409254300.000,\"threadInfo\":{\"isThreadStarving\":\"False\",\"threadWaitIntervalInMs\":0.0441,\"availableThreads\":32763,\"minThreads\":64,\"maxThreads\":32767},\"numberOfOpenTcpConnection\":517},{\"dateUtc\":\"2024-02-17T23:01:40.7637345Z\",\"cpu\":0.356,\"memory\":409247108.000,\"threadInfo\":{\"isThreadStarving\":\"False\",\"threadWaitIntervalInMs\":0.0227,\"availableThreads\":32763,\"minThreads\":64,\"maxThreads\":32767},\"numberOfOpenTcpConnection\":505},{\"dateUtc\":\"2024-02-17T23:01:50.7737451Z\",\"cpu\":0.988,\"memory\":409129648.000,\"threadInfo\":{\"isThreadStarving\":\"False\",\"threadWaitIntervalInMs\":0.0324,\"availableThreads\":32764,\"minThreads\":64,\"maxThreads\":32767},\"numberOfOpenTcpConnection\":505},{\"dateUtc\":\"2024-02-17T23:02:00.7837348Z\",\"cpu\":0.307,\"memory\":409250304.000,\"threadInfo\":{\"isThreadStarving\":\"False\",\"threadWaitIntervalInMs\":0.0647,\"availableThreads\":32764,\"minThreads\":64,\"maxThreads\":32767},\"numberOfOpenTcpConnection\":505}]}\r\nRequestStart: 2024-02-17T23:02:08.1265010Z; ResponseTime: 2024-02-17T23:02:08.1391649Z; StoreResult: StorePhysicalAddress: rntbd://cdb-ms-prod-westus3-be11.documents.azure.com:14320/apps/56ada6bd-5a60-4436-9835-23212828207c/services/f1d5c6d3-dc85-49e1-9ef8-bc161d47593a/partitions/845f30bb-1026-45d3-96c1-de139fb9e9d6/replicas/133526739009068699p/, LSN: 2, GlobalCommittedLsn: 2, PartitionKeyRangeId: 0, IsValid: True, StatusCode: 400, SubStatusCode: 1001, RequestCharge: 1.24, ItemLSN: -1, SessionToken: -1#2, UsingLocalLSN: False, TransportException: null, BELatencyMs: 0.672, ActivityId: 2757e3d0-24f7-4465-b3ec-ce6264f6c0fe, RetryAfterInMs: , ReplicaHealthStatuses: [(port: 14320 | status: Connected | lkt: 2/17/2024 11:02:08 PM)], TransportRequestTimeline: {\"requestTimeline\":[{\"event\": \"Created\", \"startTimeUtc\": \"2024-02-17T23:02:08.1265018Z\", \"durationInMs\": 0.0084},{\"event\": \"ChannelAcquisitionStarted\", \"startTimeUtc\": \"2024-02-17T23:02:08.1265102Z\", \"durationInMs\": 10.7556},{\"event\": \"Pipelined\", \"startTimeUtc\": \"2024-02-17T23:02:08.1372658Z\", \"durationInMs\": 0.0504},{\"event\": \"Transit Time\", \"startTimeUtc\": \"2024-02-17T23:02:08.1373162Z\", \"durationInMs\": 1.5947},{\"event\": \"Received\", \"startTimeUtc\": \"2024-02-17T23:02:08.1389109Z\", \"durationInMs\": 0.0842},{\"event\": \"Completed\", \"startTimeUtc\": \"2024-02-17T23:02:08.1389951Z\", \"durationInMs\": 0}],\"serviceEndpointStats\":{\"inflightRequests\":1,\"openConnections\":1},\"connectionStats\":{\"waitforConnectionInit\":\"True\",\"callsPendingReceive\":0,\"lastSendAttempt\":\"2024-02-17T23:02:08.1361548Z\",\"lastSend\":\"2024-02-17T23:02:08.1361739Z\",\"lastReceive\":\"2024-02-17T23:02:08.1369471Z\"},\"requestSizeInBytes\":567,\"requestBodySizeInBytes\":60,\"responseMetadataSizeInBytes\":224,\"responseBodySizeInBytes\":166};\r\n ResourceType: Document, OperationType: Upsert\r\n, SDK: Microsoft.Azure.Documents.Common/2.14.0") }, headers: {"x-ms-serviceversion": "version=2.14.0.0", "x-ms-current-write-quorum": "3", "x-ms-activity-id": "2757e3d0-24f7-4465-b3ec-ce6264f6c0fe", "x-ms-global-committed-lsn": "2", "server": "Microsoft-HTTPAPI/2.0", "x-ms-request-charge": "1.24", "x-ms-session-token": "0:-1#2", "x-ms-last-state-change-utc": "Sat, 17 Feb 2024 20:05:03.197 GMT", "x-ms-number-of-read-regions": "0", "x-ms-request-duration-ms": "0.672", "lsn": "2", "strict-transport-security": "max-age=31536000", "x-ms-gatewayversion": "version=2.14.0", "date": "Sat, 17 Feb 2024 23:02:07 GMT", "x-ms-current-replica-set-size": "4", "x-ms-transport-request-id": "1", "transfer-encoding": "chunked", "content-type": "application/json", "x-ms-cosmos-physical-partition-id": "6249a973-588d-4def-b3cc-3c0a08cd73dc", "x-ms-documentdb-partitionkeyrangeid": "0", "x-ms-xp-role": "1", "x-ms-cosmos-llsn": "2", "x-ms-schemaversion": "1.17", "x-ms-quorum-acked-lsn": "2", "x-ms-substatus": "1001", "x-ms-cosmos-quorum-acked-llsn": "2"}, body: b"{\"code\":\"BadRequest\",\"message\":\"Message: {\\\"Errors\\\":[\\\"PartitionKey extracted from document doesn't match the one specified in the header. Learn more: https:\\\\/\\\\/aka.ms\\\\/CosmosDB\\\\/sql\\\\/errors\\\\/wrong-pk-value\\\"]}\\r\\nActivityId: 2757e3d0-24f7-4465-b3ec-ce6264f6c0fe, Request URI: /apps/56ada6bd-5a60-4436-9835-23212828207c/services/f1d5c6d3-dc85-49e1-9ef8-bc161d47593a/partitions/845f30bb-1026-45d3-96c1-de139fb9e9d6/replicas/133526739009068699p/, RequestStats: \\r\\nRequestStartTime: 2024-02-17T23:02:08.1172474Z, RequestEndTime: 2024-02-17T23:02:08.1391649Z,  Number of regions attempted:1\\r\\n{\\\"systemHistory\\\":[{\\\"dateUtc\\\":\\\"2024-02-17T23:01:10.7337246Z\\\",\\\"cpu\\\":0.302,\\\"memory\\\":409273560.000,\\\"threadInfo\\\":{\\\"isThreadStarving\\\":\\\"False\\\",\\\"threadWaitIntervalInMs\\\":0.0466,\\\"availableThreads\\\":32760,\\\"minThreads\\\":64,\\\"maxThreads\\\":32767},\\\"numberOfOpenTcpConnection\\\":517},{\\\"dateUtc\\\":\\\"2024-02-17T23:01:20.7436998Z\\\",\\\"cpu\\\":0.539,\\\"memory\\\":409272336.000,\\\"threadInfo\\\":{\\\"isThreadStarving\\\":\\\"False\\\",\\\"threadWaitIntervalInMs\\\":0.0306,\\\"availableThreads\\\":32764,\\\"minThreads\\\":64,\\\"maxThreads\\\":32767},\\\"numberOfOpenTcpConnection\\\":517},{\\\"dateUtc\\\":\\\"2024-02-17T23:01:30.7537134Z\\\",\\\"cpu\\\":0.805,\\\"memory\\\":409254300.000,\\\"threadInfo\\\":{\\\"isThreadStarving\\\":\\\"False\\\",\\\"threadWaitIntervalInMs\\\":0.0441,\\\"availableThreads\\\":32763,\\\"minThreads\\\":64,\\\"maxThreads\\\":32767},\\\"numberOfOpenTcpConnection\\\":517},{\\\"dateUtc\\\":\\\"2024-02-17T23:01:40.7637345Z\\\",\\\"cpu\\\":0.356,\\\"memory\\\":409247108.000,\\\"threadInfo\\\":{\\\"isThreadStarving\\\":\\\"False\\\",\\\"threadWaitIntervalInMs\\\":0.0227,\\\"availableThreads\\\":32763,\\\"minThreads\\\":64,\\\"maxThreads\\\":32767},\\\"numberOfOpenTcpConnection\\\":505},{\\\"dateUtc\\\":\\\"2024-02-17T23:01:50.7737451Z\\\",\\\"cpu\\\":0.988,\\\"memory\\\":409129648.000,\\\"threadInfo\\\":{\\\"isThreadStarving\\\":\\\"False\\\",\\\"threadWaitIntervalInMs\\\":0.0324,\\\"availableThreads\\\":32764,\\\"minThreads\\\":64,\\\"maxThreads\\\":32767},\\\"numberOfOpenTcpConnection\\\":505},{\\\"dateUtc\\\":\\\"2024-02-17T23:02:00.7837348Z\\\",\\\"cpu\\\":0.307,\\\"memory\\\":409250304.000,\\\"threadInfo\\\":{\\\"isThreadStarving\\\":\\\"False\\\",\\\"threadWaitIntervalInMs\\\":0.0647,\\\"availableThreads\\\":32764,\\\"minThreads\\\":64,\\\"maxThreads\\\":32767},\\\"numberOfOpenTcpConnection\\\":505}]}\\r\\nRequestStart: 2024-02-17T23:02:08.1265010Z; ResponseTime: 2024-02-17T23:02:08.1391649Z; StoreResult: StorePhysicalAddress: rntbd://cdb-ms-prod-westus3-be11.documents.azure.com:14320/apps/56ada6bd-5a60-4436-9835-23212828207c/services/f1d5c6d3-dc85-49e1-9ef8-bc161d47593a/partitions/845f30bb-1026-45d3-96c1-de139fb9e9d6/replicas/133526739009068699p/, LSN: 2, GlobalCommittedLsn: 2, PartitionKeyRangeId: 0, IsValid: True, StatusCode: 400, SubStatusCode: 1001, RequestCharge: 1.24, ItemLSN: -1, SessionToken: -1#2, UsingLocalLSN: False, TransportException: null, BELatencyMs: 0.672, ActivityId: 2757e3d0-24f7-4465-b3ec-ce6264f6c0fe, RetryAfterInMs: , ReplicaHealthStatuses: [(port: 14320 | status: Connected | lkt: 2/17/2024 11:02:08 PM)], TransportRequestTimeline: {\\\"requestTimeline\\\":[{\\\"event\\\": \\\"Created\\\", \\\"startTimeUtc\\\": \\\"2024-02-17T23:02:08.1265018Z\\\", \\\"durationInMs\\\": 0.0084},{\\\"event\\\": \\\"ChannelAcquisitionStarted\\\", \\\"startTimeUtc\\\": \\\"2024-02-17T23:02:08.1265102Z\\\", \\\"durationInMs\\\": 10.7556},{\\\"event\\\": \\\"Pipelined\\\", \\\"startTimeUtc\\\": \\\"2024-02-17T23:02:08.1372658Z\\\", \\\"durationInMs\\\": 0.0504},{\\\"event\\\": \\\"Transit Time\\\", \\\"startTimeUtc\\\": \\\"2024-02-17T23:02:08.1373162Z\\\", \\\"durationInMs\\\": 1.5947},{\\\"event\\\": \\\"Received\\\", \\\"startTimeUtc\\\": \\\"2024-02-17T23:02:08.1389109Z\\\", \\\"durationInMs\\\": 0.0842},{\\\"event\\\": \\\"Completed\\\", \\\"startTimeUtc\\\": \\\"2024-02-17T23:02:08.1389951Z\\\", \\\"durationInMs\\\": 0}],\\\"serviceEndpointStats\\\":{\\\"inflightRequests\\\":1,\\\"openConnections\\\":1},\\\"connectionStats\\\":{\\\"waitforConnectionInit\\\":\\\"True\\\",\\\"callsPendingReceive\\\":0,\\\"lastSendAttempt\\\":\\\"2024-02-17T23:02:08.1361548Z\\\",\\\"lastSend\\\":\\\"2024-02-17T23:02:08.1361739Z\\\",\\\"lastReceive\\\":\\\"2024-02-17T23:02:08.1369471Z\\\"},\\\"requestSizeInBytes\\\":567,\\\"requestBodySizeInBytes\\\":60,\\\"responseMetadataSizeInBytes\\\":224,\\\"responseBodySizeInBytes\\\":166};\\r\\n ResourceType: Document, OperationType: Upsert\\r\\n, SDK: Microsoft.Azure.Documents.Common/2.14.0\"}" } }, "server returned error status which will not be retried: 400") }
```