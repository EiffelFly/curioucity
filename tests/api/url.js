import http from "k6/http";
import { check } from "k6";
import { API_HOST } from "./rest.js";

export const createUrl = () => {
  const url = "https://summerbud.org/id/21";

  let createUrlPayload = {
    url: url,
    resource_type: "RESOURCE_TYPE_WEBSITE",
  };

  let headers = {
    "Content-Type": "application/json",
  };

  check(
    http.request("POST", `${API_HOST}/url`, JSON.stringify(createUrlPayload), {
      headers,
    }),
    {
      "createUrl - POST /url - response status should be 201": (r) =>
        r.status === 201,
      "createUrl - POST /url - response body should have id": (r) =>
        typeof r.json().url.id !== undefined || r.json().url.id !== null,
      "createUrl - POST /url - response body should have correct url": (r) =>
        r.json().url.url === url,
      "createUrl - POST /url - response body should have correct resource_type":
        (r) => r.json().url.resource_type === createUrlPayload.resource_type,
    }
  );

  let deleteUrlPayload = {
    url,
  };

  check(
    http.request(
      "DELETE",
      `${API_HOST}/url`,
      JSON.stringify(deleteUrlPayload),
      { headers }
    ),
    {
      "createUrl - DELETE /url - response status should be 204": (r) =>
        r.status === 204,
    }
  );
};

export const deleteUrl = () => {
  const url = "https://summerbud.org/id/test";

  let createUrlPayload = {
    url: url,
    resourceType: "RESOURCE_TYPE_WEBSITE",
  };

  let headers = {
    "Content-Type": "application/json",
  };

  check(
    http.request("POST", `${API_HOST}/url`, JSON.stringify(createUrlPayload), {
      headers,
    }),
    {
      "deleteUrl - POST /url - response status should be 201": (r) =>
        r.status === 201,
    }
  );

  let deleteUrlPayload = {
    url,
  };

  check(
    http.request(
      "DELETE",
      `${API_HOST}/url`,
      JSON.stringify(deleteUrlPayload),
      { headers }
    ),
    {
      "deleteUrl - DELETE /url - response status should be 204": (r) =>
        r.status === 204,
    }
  );
};

export const getUrl = () => {
  // Should return not found when try to get not exist url
  const notExistUrl = "https://summerbud.org/id/21afr23";

  let getUrlPayload = {
    url: notExistUrl,
  };

  let headers = {
    "Content-Type": "application/json",
  };

  check(
    http.request("GET", `${API_HOST}/url`, JSON.stringify(getUrlPayload), {
      headers,
    }),
    {
      "getUrl - GET /url - not exist url, response status should be 404": (r) =>
        r.status === 404,
    }
  );

  // Should get the newly created url
  const newUrl = "https://summerbud.org/id/333423452";

  let createUrlPayload = {
    url: newUrl,
    resource_type: "RESOURCE_TYPE_WEBSITE",
  };

  check(
    http.request("POST", `${API_HOST}/url`, JSON.stringify(createUrlPayload), {
      headers,
    }),
    {
      "getUrl - POST /url - response status should be 201": (r) =>
        r.status === 201,
    }
  );

  getUrlPayload.url = newUrl;

  check(
    http.request("GET", `${API_HOST}/url`, JSON.stringify(getUrlPayload), {
      headers,
    }),
    {
      "getUrl - GET /url - response status should be 200": (r) =>
        r.status === 200,
      "getUrl - GET /url - response body should have id": (r) =>
        typeof r.json().url.id !== undefined || r.json().url.id !== null,
      "getUrl - GET /url - response body should have correct url": (r) =>
        r.json().url.url === newUrl,
      "getUrl - GET /url - response body should have correct resource_type": (
        r
      ) => r.json().url.resource_type === createUrlPayload.resource_type,
    }
  );

  let deleteUrlPayload = {
    url: newUrl,
  };

  check(
    http.request(
      "DELETE",
      `${API_HOST}/url`,
      JSON.stringify(deleteUrlPayload),
      { headers }
    ),
    {
      "getUrl - DELETE /url - response status should be 204": (r) =>
        r.status === 204,
    }
  );
};
