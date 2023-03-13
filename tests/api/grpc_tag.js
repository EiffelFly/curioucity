import grpc from "k6/net/grpc";
import { check, group } from "k6";
import { makeRandStr } from "./helper.js";
import { GRPC_API_HOST } from "./grpc.js";

const client = new grpc.Client();

client.load(
  ["../../packages/curioucity/proto"],
  "curioucity/v1alpha/tag_service.proto"
);

export const createTag = () => {
  group("gRPC TagService - Should create tag", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createTagPayload = {
      name: makeRandStr(6),
    };

    const createTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/CreateTag",
      createTagPayload
    );

    check(createTagResponse, {
      "CreateTag - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "CreateTag - response body should have id": (r) =>
        typeof r.message.tag.id !== undefined || r.message.tag.id !== null,
      "CreateTag - response body should have correct tag name": (r) =>
        r.message.tag.name === createTagPayload.name,
      "CreateTag - response body should have created_timestamp_at_curioucity": (
        r
      ) =>
        r.message.tag.createdTimestampAtCurioucity !== null ||
        r.message.tag.createdTimestampAtCurioucity !== undefined,
    });

    const deleteTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/DeleteTag",
      {
        name: createTagPayload.name,
      }
    );

    check(deleteTagResponse, {
      "CreateTag - delete test tag - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const deleteTag = () => {
  group("gRPC TagService - Should delete tag", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createTagPayload = {
      name: makeRandStr(6),
    };

    const createTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/CreateTag",
      createTagPayload
    );

    check(createTagResponse, {
      "DeleteTag - create test tag - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });

    const deleteTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/DeleteTag",
      {
        name: createTagPayload.name,
      }
    );

    check(deleteTagResponse, {
      "DeleteTag - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const getTag = () => {
  group("gRPC TagService - Should get tag", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const getNotExistTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/GetTag",
      { name: makeRandStr(6) }
    );

    check(getNotExistTagResponse, {
      "GetTag - not exist tag, response status should be StatusNotFound": (r) =>
        r.status === grpc.StatusNotFound,
    });

    const createTagPayload = {
      name: makeRandStr(6),
    };

    const createTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/CreateTag",
      createTagPayload
    );

    check(createTagResponse, {
      "GetTag - create test tag - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });

    const getNewTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/GetTag",
      { name: createTagPayload.name }
    );

    check(getNewTagResponse, {
      "GetTag - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "GetTag - response body should have id": (r) =>
        typeof r.message.tag.id !== undefined || r.message.tag.id !== null,
      "GetTag - response body should have correct tag name": (r) =>
        r.message.tag.name === createTagPayload.name,
      "GetTag - response body should have created_timestamp_at_curioucity": (
        r
      ) =>
        r.message.tag.createdTimestampAtCurioucity !== null ||
        r.message.tag.createdTimestampAtCurioucity !== undefined,
    });

    const deleteTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/DeleteTag",
      {
        name: createTagPayload.name,
      }
    );

    check(deleteTagResponse, {
      "GetTag - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const listTag = () => {
  group("gRPC TagService - Should list tags", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const newTags = [makeRandStr(6), makeRandStr(6)];

    const ListNotExistTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/ListTag",
      {}
    );

    check(ListNotExistTagResponse, {
      "ListTag - no tags exist, should response StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "ListTag - no tags exist, should response tags is empty": (r) =>
        r.message.tags.length === 0,
    });

    for (const tag of newTags) {
      const createTagResponse = client.invoke(
        "curioucity.v1alpha.TagService/CreateTag",
        {
          name: tag,
        }
      );

      check(createTagResponse, {
        "ListTag - create test tags - response status should be StatusOK": (
          r
        ) => r.status === grpc.StatusOK,
      });
    }

    const ListExistTagResponse = client.invoke(
      "curioucity.v1alpha.TagService/ListTag",
      {}
    );

    check(ListExistTagResponse, {
      "ListTag - have two tags, should response StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "ListTag - have two tags, should response size=2": (r) =>
        r.message.tags.length === newTags.length,
    });

    for (const tag of newTags) {
      const deleteTagResponse = client.invoke(
        "curioucity.v1alpha.TagService/DeleteTag",
        {
          name: tag,
        }
      );

      check(deleteTagResponse, {
        "ListTag - delete test tags - response status should be StatusOK": (
          r
        ) => r.status === grpc.StatusOK,
      });
    }
  });
};

export function cleanUpTags() {
  const listTagsResponse = client.invoke(
    "curioucity.v1alpha.TagService/ListTag",
    {}
  );

  if (
    listTagsResponse.status === grpc.StatusOK &&
    listTagsResponse.message.tags &&
    listTagsResponse.message.tags.length !== 0
  ) {
    for (const tag of listTagsResponse.message.tags) {
      const deleteTagResponse = client.invoke(
        "curioucity.v1alpha.TagService/DeleteTag",
        {
          name: tag,
        }
      );

      check(deleteTagResponse, {
        "CleanUp - clean up all tags": (r) => r.status === grpc.StatusOK,
      });
    }
  }
}
