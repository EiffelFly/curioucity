import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";
import { makeRandStr } from "./helper.js";

export const createTag = () => {
  group("Tag - Should create tag", () => {
    const tagName = makeRandStr(6);

    let createTagPayload = {
      name: tagName,
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/tags/create`,
        JSON.stringify(createTagPayload),
        {
          headers,
        }
      ),
      {
        "createTag - POST /tags/create - response status should be 201": (r) =>
          r.status === 201,
        "createTag - POST /tags/create - response body should have id": (r) =>
          typeof r.json().tag.id !== undefined || r.json().tag.id !== null,
        "createTag - POST /tags/create - response body should have correct tag name":
          (r) => r.json().tag.name === tagName,
        "createTag - POST /tags/create - response body should have created_timestamp_at_curioucity":
          (r) =>
            r.json().tag.created_timestamp_at_curioucity !== null ||
            r.json().tag.created_timestamp_at_curioucity !== undefined,
      }
    );

    check(
      http.request("DELETE", `${API_HOST}/tags/${tagName}`, undefined, {
        headers,
      }),
      {
        "createTag - DELETE /tags/:name - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const deleteTag = () => {
  group("Tag - Should delete tag", () => {
    const tagName = makeRandStr(6);

    let createTagPayload = {
      name: tagName,
    };

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/tags/create`,
        JSON.stringify(createTagPayload),
        {
          headers,
        }
      ),
      {
        "deleteTag - POST /tags/create - response status should be 201": (r) =>
          r.status === 201,
      }
    );

    check(
      http.request("DELETE", `${API_HOST}/tags/${tagName}`, undefined, {
        headers,
      }),
      {
        "deleteTag - DELETE /tags/:name - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const getTag = () => {
  group("Tag - Should get tag", () => {
    // Should return not found when try to get not exist tag
    const notExistTag = makeRandStr(6);

    let headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request("GET", `${API_HOST}/tags/${notExistTag}`, undefined, {
        headers,
      }),
      {
        "getTag - GET /tags/:name - not exist tag, response status should be 404":
          (r) => r.status === 404,
      }
    );

    // Should get the newly created tag
    const newTagName = makeRandStr(6);

    let createTagPayload = {
      name: newTagName,
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/tags/create`,
        JSON.stringify(createTagPayload),
        {
          headers,
        }
      ),
      {
        "getTag - POST /tags/create - response status should be 201": (r) =>
          r.status === 201,
      }
    );

    check(
      http.request("GET", `${API_HOST}/tags/${newTagName}`, undefined, {
        headers,
      }),
      {
        "getTag - GET /tags/:name - response status should be 200": (r) =>
          r.status === 200,
        "getTag - GET /tags/:name - response body should have id": (r) =>
          typeof r.json().tag.id !== undefined || r.json().tag.id !== null,
        "getTag - GET /tags/:name - response body should have correct tag name":
          (r) => r.json().tag.name === newTagName,
        "getTag - GET /tags/:name - response body should have created_timestamp_at_curioucity":
          (r) =>
            r.json().tag.created_timestamp_at_curioucity !== null ||
            r.json().tag.created_timestamp_at_curioucity !== undefined,
      }
    );

    check(
      http.request("DELETE", `${API_HOST}/tags/${newTagName}`, undefined, {
        headers,
      }),
      {
        "getTag - DELETE /tags/:name - response status should be 204": (r) =>
          r.status === 204,
      }
    );
  });
};

export const listTag = () => {
  group("Should list tags", () => {
    const newTags = [makeRandStr(6), makeRandStr(6)];

    let headers = {
      "Content-Type": "application/json",
    };

    check(http.request("GET", `${API_HOST}/tags`, undefined, { headers }), {
      "listTag - GET /tags - no tags exist, should response 200": (r) =>
        r.status === 200,
    });

    for (const tag of newTags) {
      let createTagPayload = {
        name: tag,
      };

      check(
        http.request(
          "POST",
          `${API_HOST}/tags/create`,
          JSON.stringify(createTagPayload),
          {
            headers,
          }
        ),
        {
          "listTag - POST /tags/create - response status should be 201": (r) =>
            r.status === 201,
        }
      );
    }

    check(http.request("GET", `${API_HOST}/tags`, undefined, { headers }), {
      "listTag - GET /tags - have two tags, should response 200": (r) =>
        r.status === 200,
    });

    for (const tag of newTags) {
      let deleteTagPayload = {
        name: tag,
      };

      check(
        http.request("DELETE", `${API_HOST}/tags/${tag}`, undefined, {
          headers,
        }),
        {
          "lsitTag - DELETE /tags/:name - response status should be 204": (r) =>
            r.status === 204,
        }
      );
    }
  });
};
