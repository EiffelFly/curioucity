import http from "k6/http";
import { check } from "k6";

export const createUrl = () => {
  let payload = {
    url: "https://summerbud.org",
    resource_type: "BlogPost",
  };

  check(http.request("POST", "", payload), {
    "POST /url response status": (r) => r.status === 201,
  });
};
