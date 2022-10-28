import http from "k6/http";
import { check } from "k6";
import { API_HOST } from "./rest.js";

export const createUrl = () => {
  let payload = {
    url: "https://summerbud.org",
    resource_type: "BlogPost",
  };

  check(http.request("POST", `${API_HOST}/url`, payload), {
    "POST /url response status": (r) => r.status === 201,
  });
};
