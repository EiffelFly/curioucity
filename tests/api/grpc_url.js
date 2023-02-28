import grpc from "k6/net/grpc";
import { check, group } from "k6";
import { makeRandStr } from "./helper.js";
import { GRPC_API_HOST } from "./grpc.js";

const client = new grpc.Client();
client.load(
  ["../../packages/curioucity/proto"],
  "curioucity/v1alpha/url_service.proto"
);

export const createUrl = () => {
  group("Url - Should create url", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const urlFrag = makeRandStr(6);
    const createUrlPayload = {
      url: `https://www.curioucity.org/${urlFrag}`,
      resource_type: "RESOURCE_TYPE_WEBSITE",
    };

    const createUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/CreateUrl",
      createUrlPayload
    );

    check(createUrlResponse, {
      "createUrl - POST /urls/create - response status should be 201": (r) =>
        r.status === grpc.StatusOK,
      "createUrl - POST /urls/create - response body should have id": (r) =>
        typeof r.message.url.id !== undefined || r.message.url.id !== null,
      "createUrl - POST /urls/create - response body should have correct url": (
        r
      ) => r.message.url.url === createUrlPayload.url,
      "createUrl - POST /urls/create - response body should have correct resource_type":
        (r) => r.message.url.resourceType === createUrlPayload.resource_type,
      "createUrl - POST /urls/create - response body should have created_timestamp_at_curioucity":
        (r) =>
          r.message.url.createdTimestampAtCurioucity !== null ||
          r.message.url.createdTimestampAtCurioucity !== undefined,
    });

    const deleteUrlResponse = client.invoke(
      "url_service.UrlService/DeleteUrl",
      {
        url: createUrlPayload.url,
      }
    );

    check(deleteUrlResponse, {
      "createUrl - DELETE /urls/:url - response status should be 204": (r) =>
        r.status === 204,
    });
  });
};
