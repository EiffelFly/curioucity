# Curioucity

## Preface

At first, I built this tool to wire the Discord discussion to be searchable, the process is quite simple, Install a bot in the Discord server with a custom command, and once you use the command on the Discord thread, it will wrap the whole discussion thread and send it to the curioucity server. At the server, curioucity will parse the thread with the help of remark to form a proper AST(Abstract syntax tree) and store them in the edgeDB.

After this step, Curioucity will trigger the event to re-build the Nextjs frontend (Ideally you could put your frontend code in a repo, and the curioucity server will post a request to the repo and open a separate PR).

## Technology highlight

To make our API consistent, I am using protobuf to government the API for both gRPC and REST endpoint. There have lots of challenges within this step. 

- I use buf.build to maintain the integrity of the protobuf code
- I use [protoc-gen-prost](https://github.com/neoeinstein/protoc-gen-prost) to generate the desired types included JSON serializer 
- I use tokio as gRPC server, axum as REST server then bind then with tower::Steep (This is the most challenging part)
- To use edgeDB, I write a bunch of wrappers on top of it rust crate
- I use k6 to do the API integration-test

## Project status

Right now, the project is stale, I can't find further motivation to push it forward. I will leave this project as is to serve as a playground/example about how to build a server with protobuf and wrap REST and gRPC endpoint around it. Overall, this is a great adventure.

## Philosophy

- [On a way to solve clustered context, my thinking process](https://www.summerbud.org/thoughts/my-thinking-process-of-solving-clustered-context#user-content-fnref-3)
- [Why is the documentation of tech products so hard to use? (In the user’s point of view)](https://www.summerbud.org/thoughts/why-is-the-documentation-of-tech-products-so-hard-to-use)
