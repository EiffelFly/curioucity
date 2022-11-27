import http from "k6/http";
import { check } from "k6";
import { API_HOST } from "./rest.js";

export const createTag = () => {
  const tagName = "knowledge-management-toolkit";

  let createTagPayload = {
    name: tagName,
  };

  let headers = {
    "Content-Type": "application/json",
  };

  check(
    http.request("POST", `${API_HOST}/tag`, JSON.stringify(createTagPayload), {
      headers,
    }),
    {
      "createTag - POST /tag - response status should be 201": (r) =>
        r.status === 201,
      "createTag - POST /tag - response body should have id": (r) =>
        typeof r.json().tag.id !== undefined || r.json().tag.id !== null,
      "createTag - POST /tag - response body should have correct tag name": (
        r
      ) => r.json().tag.name === tagName,
    }
  );

  let deleteTagPayload = {
    name: tagName,
  };

  check(
    http.request(
      "DELETE",
      `${API_HOST}/tag`,
      JSON.stringify(deleteTagPayload),
      { headers }
    ),
    {
      "createTag - DELETE /tag - response status should be 204": (r) =>
        r.status === 204,
    }
  );
};

export const deleteTag = () => {
  const tagName = "knowledge-management-toolkit";

  let createTagPayload = {
    name: tagName,
  };

  let headers = {
    "Content-Type": "application/json",
  };

  check(
    http.request("POST", `${API_HOST}/tag`, JSON.stringify(createTagPayload), {
      headers,
    }),
    {
      "deleteTag - POST /tag - response status should be 201": (r) =>
        r.status === 201,
    }
  );

  let deleteTagPayload = {
    name: tagName,
  };

  check(
    http.request(
      "DELETE",
      `${API_HOST}/tag`,
      JSON.stringify(deleteTagPayload),
      { headers }
    ),
    {
      "deleteTag - DELETE /tag - response status should be 204": (r) =>
        r.status === 204,
    }
  );
};

export const getTag = () => {
  // Should return not found when try to get not exist tag
  const notExistTag = "web3";

  let getTagPayload = {
    name: notExistTag,
  };

  let headers = {
    "Content-Type": "application/json",
  };

  check(
    http.request("GET", `${API_HOST}/tag`, JSON.stringify(getTagPayload), {
      headers,
    }),
    {
      "getTag - GET /tag - not exist tag, response status should be 404": (r) =>
        r.status === 404,
    }
  );

  // Should get the newly created tag
  const newTagName = "knowledge-management-toolkit";

  let createTagPayload = {
    name: newTagName,
  };

  check(
    http.request("POST", `${API_HOST}/tag`, JSON.stringify(createTagPayload), {
      headers,
    }),
    {
      "getTag - POST /tag - response status should be 201": (r) =>
        r.status === 201,
    }
  );

  getTagPayload.name = newTagName;

  check(
    http.request("GET", `${API_HOST}/tag`, JSON.stringify(getTagPayload), {
      headers,
    }),
    {
      "getTag - GET /tag - response status should be 200": (r) =>
        r.status === 200,
      "getTag - GET /tag - response body should have id": (r) =>
        typeof r.json().tag.id !== undefined || r.json().tag.id !== null,
      "getTag - GET /tag - response body should have correct tag name": (r) =>
        r.json().tag.name === newTagName,
    }
  );

  let deleteTagPayload = {
    name: newTagName,
  };

  check(
    http.request(
      "DELETE",
      `${API_HOST}/tag`,
      JSON.stringify(deleteTagPayload),
      { headers }
    ),
    {
      "getTag - DELETE /tag - response status should be 204": (r) =>
        r.status === 204,
    }
  );
};
