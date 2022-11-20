import http from "k6/http";
import { check } from "k6";
import { API_HOST } from "./rest.js";

export const createUrl = () => {
  const url = "https://summerbud.org/id/21";

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
      "POST /url response status": (r) => r.status === 201,
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
      "DELETE /url response status": (r) => r.status === 204,
    }
  );
};

export const getUrl = () => {
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
      "GET /url not exist response status": (r) => r.status === 404,
    }
  );
};
