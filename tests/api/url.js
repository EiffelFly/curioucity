import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";

export const createUrl = () => {
  group("Url - Should create url", () => {
    const url = "https://summerbud.org/id/234azyyq456";

    let createUrlPayload = {
      url: url,
      resource_type: "RESOURCE_TYPE_WEBSITE",
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/urls`,
        JSON.stringify(createUrlPayload),
        {
          headers,
        }
      ),
      {
        "createUrl - POST /urls - response status should be 201": (r) =>
          r.status === 201,
        "createUrl - POST /urls - response body should have id": (r) =>
          typeof r.json().url.id !== undefined || r.json().url.id !== null,
        "createUrl - POST /urls - response body should have correct url": (r) =>
          r.json().url.url === url,
        "createUrl - POST /urls - response body should have correct resource_type":
          (r) => r.json().url.resource_type === createUrlPayload.resource_type,
        "createUrl - POST /urls - response body should have created_timestamp_at_curioucity":
          (r) =>
            r.json().url.created_timestamp_at_curioucity !== null ||
            r.json().url.created_timestamp_at_curioucity !== undefined,
      }
    );

    let deleteUrlPayload = {
      url,
    };

    check(
      http.request(
        "DELETE",
        `${API_HOST}/urls`,
        JSON.stringify(deleteUrlPayload),
        { headers }
      ),
      {
        "createUrl - DELETE /urls - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const deleteUrl = () => {
  group("Url - Should delete url", () => {
    const url = "https://summerbud.org/id/test";

    let createUrlPayload = {
      url: url,
      resourceType: "RESOURCE_TYPE_WEBSITE",
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/urls`,
        JSON.stringify(createUrlPayload),
        {
          headers,
        }
      ),
      {
        "deleteUrl - POST /urls - response status should be 201": (r) =>
          r.status === 201,
      }
    );

    let deleteUrlPayload = {
      url,
    };

    check(
      http.request(
        "DELETE",
        `${API_HOST}/urls`,
        JSON.stringify(deleteUrlPayload),
        { headers }
      ),
      {
        "deleteUrl - DELETE /urls - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const getUrl = () => {
  group("Url - Should get url", () => {
    // Should return not found when try to get not exist url
    const notExistUrl = "https://summerbud.org/id/21afr23";

    let headers = {
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
        "getUrl - GET /urls - not exist url, response status should be 404": (
          r
        ) => r.status === 404,
      }
    );

    // Should get the newly created url
    const newUrl = "https://summerbud.org/id/305968";

    let createUrlPayload = {
      url: newUrl,
      resource_type: "RESOURCE_TYPE_WEBSITE",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/urls`,
        JSON.stringify(createUrlPayload),
        {
          headers,
        }
      ),
      {
        "getUrl - POST /urls - response status should be 201": (r) =>
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
        "getUrl - GET /urls - response status should be 200": (r) =>
          r.status === 200,
        "getUrl - GET /urls - response body should have id": (r) =>
          typeof r.json().url.id !== undefined || r.json().url.id !== null,
        "getUrl - GET /urls - response body should have correct url": (r) =>
          r.json().url.url === newUrl,
        "getUrl - GET /urls - response body should have correct resource_type":
          (r) => r.json().url.resource_type === createUrlPayload.resource_type,
        "getUrl - GET /urls - response body should have correct created_timestamp_at_curioucity":
          (r) =>
            r.json().url.created_timestamp_at_curioucity !== null ||
            r.json().url.created_timestamp_at_curioucity !== undefined,
      }
    );

    let deleteUrlPayload = {
      url: newUrl,
    };

    check(
      http.request(
        "DELETE",
        `${API_HOST}/urls`,
        JSON.stringify(deleteUrlPayload),
        { headers }
      ),
      {
        "getUrl - DELETE /urls - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const listUrl = () => {
  group("Should list tags", () => {
    const newUrls = [
      "https://www.summerbud.org/id/258",
      "https://www.summerbud.org/id/2675",
    ];

    let headers = {
      "Content-Type": "application/json",
    };

    check(http.request("GET", `${API_HOST}/urls`, undefined, { headers }), {
      "listUrl - GET /urls - no urls exist, should response 200": (r) =>
        r.status === 200,
    });

    for (const url of newUrls) {
      let createUrlPayload = {
        url,
        resource_type: "RESOURCE_TYPE_WEBSITE",
      };

      check(
        http.request(
          "POST",
          `${API_HOST}/urls`,
          JSON.stringify(createUrlPayload),
          {
            headers,
          }
        ),
        {
          "listUrl - POST /urls - response status should be 201": (r) =>
            r.status === 201,
        }
      );
    }

    check(http.request("GET", `${API_HOST}/urls`, undefined, { headers }), {
      "listUrl - GET /urls - have two urls, should response 200": (r) =>
        r.status === 200,
      "listUrl - GET /urls - have two urls, should response size=2": (r) =>
        r.json().size === "2",
    });

    for (const url of newUrls) {
      let deleteUrlPayload = {
        url,
      };

      check(
        http.request(
          "DELETE",
          `${API_HOST}/urls`,
          JSON.stringify(deleteUrlPayload),
          {
            headers,
          }
        ),
        {
          "listUrl - DELETE /urls - response status should be 204": (r) =>
            r.status === 204,
        }
      );
    }
  });
};
