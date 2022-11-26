export const createTag = () => {
  const name = "knowledge-management";

  let createTagPayload = {
    name,
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
      "createTag - POST /tag - response body should have correct name": (r) =>
        r.json().tag.name === tag,
    }
  );

  let deleteTagPayload = {
    tag,
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
  const name = "knowledge-management";

  let createTagPayload = {
    name,
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
    tag,
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
    http.request("GET", `${API_HOST}/url`, JSON.stringify(getTagPayload), {
      headers,
    }),
    {
      "getUrl - GET /url - not exist tag, response status should be 404": (r) =>
        r.status === 404,
    }
  );

  // Should get the newly created tag
  const newTag = "knowledge-management";

  let createTagPayload = {
    name: newTag,
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

  getTagPayload.url = newTag;

  check(
    http.request("GET", `${API_HOST}/tag`, JSON.stringify(getTagPayload), {
      headers,
    }),
    {
      "getTag - GET /tag - response status should be 200": (r) =>
        r.status === 200,
      "getTag - GET /tag - response body should have id": (r) =>
        typeof r.json().url.id !== undefined || r.json().url.id !== null,
      "getTag - GET /tag - response body should have correct tag name": (r) =>
        r.json().tag.name === newTag,
    }
  );

  let deleteTagPayload = {
    name: newTag,
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
