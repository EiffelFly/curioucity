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
  group("gRPC UrlService - Should create url", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const createUrlPayload = {
      url: `https://www.curioucity.org/${makeRandStr(6)}`,
      resource_type: "RESOURCE_TYPE_WEBSITE",
    };

    const createUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/CreateUrl",
      createUrlPayload
    );

    check(createUrlResponse, {
      "CreateUrl - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "CreateUrl - response body should have id": (r) =>
        typeof r.message.url.id !== undefined || r.message.url.id !== null,
      "CreateUrl - response body should have correct url": (r) =>
        r.message.url.url === createUrlPayload.url,
      "CreateUrl - response body should have correct resource_type": (r) =>
        r.message.url.resourceType === createUrlPayload.resource_type,
      "CreateUrl - response body should have created_timestamp_at_curioucity": (
        r
      ) =>
        r.message.url.createdTimestampAtCurioucity !== null ||
        r.message.url.createdTimestampAtCurioucity !== undefined,
    });

    const deleteUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/DeleteUrl",
      {
        url: createUrlPayload.url,
      }
    );

    check(deleteUrlResponse, {
      "CreateUrl - clean up created url - response status should be StatusOK": (
        r
      ) => r.status === grpc.StatusOK,
    });
  });
};

export const deleteUrl = () => {
  group("gRPC UrlService - Should delete url", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const createUrlPayload = {
      url: `https://www.curioucity.org/${makeRandStr(6)}`,
      resourceType: "RESOURCE_TYPE_WEBSITE",
    };

    const createUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/CreateUrl",
      createUrlPayload
    );

    check(createUrlResponse, {
      "DeleteUrl - create test url - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });

    const deleteUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/DeleteUrl",
      {
        url: createUrlPayload.url,
      }
    );

    check(deleteUrlResponse, {
      "DeleteUrl - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const getUrl = () => {
  group("gRPC UrlService - Should get url", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    // Should return not found when try to get not exist url
    const notExistUrl = `https://www.curioucity.org/${makeRandStr(6)}`;

    const getNotExistUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/GetUrl",
      {
        url: notExistUrl,
      }
    );

    check(getNotExistUrlResponse, {
      "GetUrl - not exist url, response status should be 404": (r) =>
        r.status === grpc.StatusNotFound,
    });

    // Should get the newly created url
    const createNewUrlPayload = {
      url: `https://www.curioucity.org/${makeRandStr(6)}`,
      resource_type: "RESOURCE_TYPE_WEBSITE",
    };

    const createNewUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/CreateUrl",
      createNewUrlPayload
    );

    check(createNewUrlResponse, {
      "GetUrl - create test url - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });

    const getNewUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/GetUrl",
      {
        url: createNewUrlPayload.url,
      }
    );

    check(getNewUrlResponse, {
      "GetUrl - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "GetUrl - response body should have id": (r) =>
        typeof r.message.url.id !== undefined || r.json().url.id !== null,
      "GetUrl - response body should have correct url": (r) =>
        r.message.url.url === createNewUrlPayload.url,
      "GetUrl - response body should have correct resource_type": (r) =>
        r.message.url.resourceType === createNewUrlPayload.resource_type,
      "GetUrl - response body should have correct created_timestamp_at_curioucity":
        (r) =>
          r.message.url.createdTimestampAtCurioucity !== null ||
          r.message.url.createdTimestampAtCurioucity !== undefined,
    });

    const deleteNewUrlResponse = client.invoke(
      "curioucity.v1alpha.UrlService/DeleteUrl",
      {
        url: createNewUrlPayload.url,
      }
    );

    check(deleteNewUrlResponse, {
      "GetUrl - clean up created url - response status should be StatusOK": (
        r
      ) => r.status === grpc.StatusOK,
    });
  });
};

export const listUrl = () => {
  group("Should list urls", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const newUrls = [
      `https://www.curioucity.org/${makeRandStr(6)}`,
      `https://www.curioucity.org/${makeRandStr(6)}`,
    ];

    const ListUrlWithoutUrlsResponse = client.invoke(
      "curioucity.v1alpha.UrlService/ListUrl",
      {}
    );

    check(ListUrlWithoutUrlsResponse, {
      "ListUrl - no urls exist, should response StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "ListUrl - no urls exist, should response urls is empty": (r) =>
        r.message.urls.length === 0,
    });

    for (const url of newUrls) {
      const createNewUrlPayload = {
        url,
        resource_type: "RESOURCE_TYPE_WEBSITE",
      };

      const createNewUrlResponse = client.invoke(
        "curioucity.v1alpha.UrlService/CreateUrl",
        createNewUrlPayload
      );

      check(createNewUrlResponse, {
        "ListUrl - create test urls - response status should be StatusOK": (
          r
        ) => r.status === grpc.StatusOK,
      });
    }

    const ListUrlWithUrlsResponse = client.invoke(
      "curioucity.v1alpha.UrlService/ListUrl",
      {}
    );

    check(ListUrlWithUrlsResponse, {
      "ListUrl - have two urls, should response StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "listUrl - have two urls, should response size=2": (r) =>
        r.message.size === "2",
    });

    for (const url of newUrls) {
      const deleteNewUrlResponse = client.invoke(
        "curioucity.v1alpha.UrlService/DeleteUrl",
        {
          url,
        }
      );

      check(deleteNewUrlResponse, {
        "listUrl - clean up created url - response status should be StatusOK": (
          r
        ) => r.status === grpc.StatusOK,
      });
    }
  });
};
