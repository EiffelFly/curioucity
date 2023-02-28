import * as urlServices from "./grpc_url.js";

export const GRPC_API_HOST = "localhost:8080";

const grpc = () => {
  urlServices.createUrl();
};

export default grpc;
