import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";

export const createTag = () => {
  group("Tag - Should create tag", () => {
    const tagName = "knowledge-management-toolkit-22";

    let createTagPayload = {
      name: tagName,
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/tags`,
        JSON.stringify(createTagPayload),
        {
          headers,
        }
      ),
      {
        "createTag - POST /tags - response status should be 201": (r) =>
          r.status === 201,
        "createTag - POST /tags - response body should have id": (r) =>
          typeof r.json().tag.id !== undefined || r.json().tag.id !== null,
        "createTag - POST /tags - response body should have correct tag name": (
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
        `${API_HOST}/tags`,
        JSON.stringify(deleteTagPayload),
        { headers }
      ),
      {
        "createTag - DELETE /tags - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const deleteTag = () => {
  group("Tag - Should delete tag", () => {
    const tagName = "knowledge-management-toolkit-34";

    let createTagPayload = {
      name: tagName,
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/tags`,
        JSON.stringify(createTagPayload),
        {
          headers,
        }
      ),
      {
        "deleteTag - POST /tags - response status should be 201": (r) =>
          r.status === 201,
      }
    );

    let deleteTagPayload = {
      name: tagName,
    };

    check(
      http.request(
        "DELETE",
        `${API_HOST}/tags`,
        JSON.stringify(deleteTagPayload),
        { headers }
      ),
      {
        "deleteTag - DELETE /tags - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const getTag = () => {
  group("Tag - Should get tag", () => {
    // Should return not found when try to get not exist tag
    const notExistTag = "web3";

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request("GET", `${API_HOST}/tags/${notExistTag}`, undefined, {
        headers,
      }),
      {
        "getTag - GET /tags/{name} - not exist tag, response status should be 404":
          (r) => r.status === 404,
      }
    );

    // Should get the newly created tag
    const newTagName = "knowledge-management-toolkit-123";

    let createTagPayload = {
      name: newTagName,
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/tags`,
        JSON.stringify(createTagPayload),
        {
          headers,
        }
      ),
      {
        "getTag - POST /tags - response status should be 201": (r) =>
          r.status === 201,
      }
    );

    check(
      http.request("GET", `${API_HOST}/tags/${newTagName}`, undefined, {
        headers,
      }),
      {
        "getTag - GET /tags/{name} - response status should be 200": (r) =>
          r.status === 200,
        "getTag - GET /tags/{name} - response body should have id": (r) =>
          typeof r.json().tag.id !== undefined || r.json().tag.id !== null,
        "getTag - GET /tags/{name} - response body should have correct tag name":
          (r) => r.json().tag.name === newTagName,
      }
    );

    let deleteTagPayload = {
      name: newTagName,
    };

    check(
      http.request(
        "DELETE",
        `${API_HOST}/tags`,
        JSON.stringify(deleteTagPayload),
        { headers }
      ),
      {
        "getTag - DELETE /tags - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};
