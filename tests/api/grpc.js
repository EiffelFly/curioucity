import * as urlServices from "./grpc_url.js";
import * as tagServices from "./grpc_tag.js";

export const GRPC_API_HOST = "localhost:8080";

const grpc = () => {
  // urlServices.createUrl();
  // urlServices.deleteUrl();
  // urlServices.getUrl();
  // urlServices.listUrl();
  // tagServices.createTag();
  // tagServices.deleteTag();
  tagServices.getTag();
};

export default grpc;
