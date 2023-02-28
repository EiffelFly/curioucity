### About the k6 gRPC response is camelCase instead of snake_case

Currently, there have no other ways to configure the k6 gRPC response snake_case property. We have to alter the test script to cope with this, [reference](https://community.k6.io/t/grpc-response-is-camelcase-instead-of-snake-case/3266)

### Caveat

When load the proto definition file the path is relative from the triggered script to the proto not from the test script to the proto


├── proto
├── tests
├── main.js <-- trigger file that trigger the whole test
└── grpc
    └── grpc_url.js

In the `/grpc/grpc.url.js` the client load script should be 

```js
client.load(
  ["./proto"],
  "url_service.proto"
);
```

instead of 

```js
client.load(
  ["../../proto"],
  "url_service.proto"
);
```