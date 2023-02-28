import * as urlServices from "./grpc_url.js";

export const GRPC_API_HOST = "localhost:8080";

const grpc = () => {
  urlServices.createUrl();
  urlServices.deleteUrl();
  urlServices.getUrl();
  urlServices.listUrl();
};

export default grpc;
