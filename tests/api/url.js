import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";
import { makeRandStr } from "./helper.js";

export const createUrl = () => {
  group("Url - Should create url", () => {
    const urlFrag = makeRandStr(6);

    const createUrlPayload = {
      url: `https://www.curioucity.org/${urlFrag}`,
      resource_type: "RESOURCE_TYPE_WEBSITE",
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/urls/create`,
        JSON.stringify(createUrlPayload),
        {
          headers,
        }
      ),
      {
        "createUrl - POST /urls/create - response status should be 201": (r) =>
          r.status === 201,
        "createUrl - POST /urls/create - response body should have id": (r) =>
          typeof r.json().url.id !== undefined || r.json().url.id !== null,
        "createUrl - POST /urls/create - response body should have correct url":
          (r) => r.json().url.url === url,
        "createUrl - POST /urls/create - response body should have correct resource_type":
          (r) => r.json().url.resource_type === createUrlPayload.resource_type,
        "createUrl - POST /urls/create - response body should have created_timestamp_at_curioucity":
          (r) =>
            r.json().url.created_timestamp_at_curioucity !== null ||
            r.json().url.created_timestamp_at_curioucity !== undefined,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/urls/${encodeURIComponent(createUrlPayload.url)}`,
        undefined,
        { headers }
      ),
      {
        "createUrl - DELETE /urls/:url - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const deleteUrl = () => {
  group("Url - Should delete url", () => {
    const url = makeRandStr(6);

    const createUrlPayload = {
      url: url,
      resourceType: "RESOURCE_TYPE_WEBSITE",
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/urls/create`,
        JSON.stringify(createUrlPayload),
        {
          headers,
        }
      ),
      {
        "deleteUrl - POST /urls/create - response status should be 201": (r) =>
          r.status === 201,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/urls/${encodeURIComponent(url)}`,
        undefined,
        { headers }
      ),
      {
        "deleteUrl - DELETE /urls/:url - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const getUrl = () => {
  group("Url - Should get url", () => {
    // Should return not found when try to get not exist url
    const notExistUrl = makeRandStr(6);

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "GET",
        `${API_HOST}/urls/${encodeURIComponent(notExistUrl)}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getUrl - GET /urls/:url - not exist url, response status should be 404":
          (r) => r.status === 404,
      }
    );

    // Should get the newly created url
    const newUrl = makeRandStr(6);

    const createUrlPayload = {
      url: newUrl,
      resource_type: "RESOURCE_TYPE_WEBSITE",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/urls/create`,
        JSON.stringify(createUrlPayload),
        {
          headers,
        }
      ),
      {
        "getUrl - POST /urls/create - response status should be 201": (r) =>
          r.status === 201,
      }
    );

    check(
      http.request(
        "GET",
        `${API_HOST}/urls/${encodeURIComponent(newUrl)}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getUrl - GET /urls/:url - response status should be 200": (r) =>
          r.status === 200,
        "getUrl - GET /urls/:url - response body should have id": (r) =>
          typeof r.json().url.id !== undefined || r.json().url.id !== null,
        "getUrl - GET /urls/:url - response body should have correct url": (
          r
        ) => r.json().url.url === newUrl,
        "getUrl - GET /urls/:url - response body should have correct resource_type":
          (r) => r.json().url.resource_type === createUrlPayload.resource_type,
        "getUrl - GET /urls/:url - response body should have correct created_timestamp_at_curioucity":
          (r) =>
            r.json().url.created_timestamp_at_curioucity !== null ||
            r.json().url.created_timestamp_at_curioucity !== undefined,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/urls/${encodeURIComponent(newUrl)}`,
        undefined,
        { headers }
      ),
      {
        "getUrl - DELETE /urls/:url - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const listUrl = () => {
  group("Should list urls", () => {
    const newUrls = [makeRandStr(6), makeRandStr(6)];

    const headers = {
      "Content-Type": "application/json",
    };

    check(http.request("GET", `${API_HOST}/urls`, undefined, { headers }), {
      "listUrl - GET /urls - no urls exist, should response 200": (r) =>
        r.status === 200,
    });

    for (const url of newUrls) {
      const createUrlPayload = {
        url,
        resource_type: "RESOURCE_TYPE_WEBSITE",
      };

      check(
        http.request(
          "POST",
          `${API_HOST}/urls/create`,
          JSON.stringify(createUrlPayload),
          {
            headers,
          }
        ),
        {
          "listUrl - POST /urls/create - response status should be 201": (r) =>
            r.status === 201,
        }
      );
    }

    // Need to find another way to test this

    // check(http.request("GET", `${API_HOST}/urls`, undefined, { headers }), {
    //   "listUrl - GET /urls - have two urls, should response 200": (r) =>
    //     r.status === 200,
    //   "listUrl - GET /urls - have two urls, should response size=2": (r) =>
    //     r.json().size === "2",
    // });

    for (const url of newUrls) {
      check(
        http.request(
          "DELETE",
          `${API_HOST}/urls/${encodeURIComponent(url)}`,
          undefined,
          {
            headers,
          }
        ),
        {
          "listUrl - DELETE /urls/:url - response status should be 204": (r) =>
            r.status === 204,
        }
      );
    }
  });
};
