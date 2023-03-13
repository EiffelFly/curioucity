import grpc from "k6/net/grpc";
import { check, group } from "k6";
import { GRPC_API_HOST } from "./grpc.js";

const client = new grpc.Client();

client.load(
  ["../../packages/curioucity/proto"],
  "third_party/v1alpha/discord_service.proto"
);

export const createDiscordThread = () => {
  group("gRPC DiscordService - Should create discord thread", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createDiscordThreadPayload = {
      thread_id: `${Math.floor(Math.random() * 100000000)}`,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const createDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordThread",
      createDiscordThreadPayload
    );

    check(createDiscordThreadResponse, {
      "CreateDiscordThread - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "CreateDiscordThread - response body should have id": (r) =>
        typeof r.message.discordThread.id !== undefined &&
        r.message.discordThread.id !== null,
      "CreateDiscordThread - response body should have correct thread_id": (
        r
      ) =>
        r.message.discordThread.threadId ===
        createDiscordThreadPayload.thread_id.toString(),
      "CreateDiscordThread - response body should have correct markdown content":
        (r) =>
          r.message.discordThread.markdownContent ===
          createDiscordThreadPayload.markdown_content,
      "CreateDiscordThread - response body should have correct url": (r) =>
        r.message.discordThread.url.url === createDiscordThreadPayload.url,
      "CreateDiscordThread - response body should have correct created_timestamp_at_discord":
        (r) =>
          Date.parse(r.message.discordThread.createdTimestampAtDiscord) /
            1000 ===
          createDiscordThreadPayload.created_timestamp_at_discord,
      "CreateDiscordThread - response body should have created_timestamp_at_curioucity":
        (r) =>
          typeof r.message.discordThread.createdTimestampAtCurioucity !==
            "undefined" &&
          r.message.discordThread.created_timestamp_at_curioucity !== null,
    });

    const deleteDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordThread",
      {
        thread_id: createDiscordThreadPayload.thread_id,
      }
    );

    check(deleteDiscordThreadResponse, {
      "CreateDiscordThread - delete test discord thread - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });
  });
};

export const deleteDiscordThread = () => {
  group("gRPC DiscordService - Should delete disocrd thread", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createDiscordThreadPayload = {
      thread_id: `${Math.floor(Math.random() * 100000000)}`,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const createDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordThread",
      createDiscordThreadPayload
    );

    check(createDiscordThreadResponse, {
      "DeleteDiscordThread - create test discord thread - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });

    const deleteDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordThread",
      {
        thread_id: createDiscordThreadPayload.thread_id,
      }
    );

    check(deleteDiscordThreadResponse, {
      "DeleteDiscordThread - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const getDiscordThread = () => {
  group("gRPC DiscordService - Should get discord thread", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const getNotExistDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/GetDiscordThread",
      {
        thread_id: `${Math.floor(Math.random() * 100000000)}`,
      }
    );

    check(getNotExistDiscordThreadResponse, {
      "GetDiscordThread - get not exist discord thread, response status should be StatusNotFound":
        (r) => r.status === grpc.StatusNotFound,
    });

    const createDiscordThreadPayload = {
      thread_id: `${Math.floor(Math.random() * 100000000)}`,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const createDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordThread",
      createDiscordThreadPayload
    );

    check(createDiscordThreadResponse, {
      "GetDiscordThread - create test discord thread - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });

    const getExistDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/GetDiscordThread",
      {
        thread_id: createDiscordThreadPayload.thread_id,
      }
    );

    check(getExistDiscordThreadResponse, {
      "GetDiscordThread - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "GetDiscordThread - response body should have id": (r) =>
        typeof r.message.discordThread.id !== undefined &&
        r.message.discordThread.id !== null,
      "GetDiscordThread - GET /discord/threads/:thread_id - response body should have correct thread_id":
        (r) =>
          r.message.discordThread.threadId ===
          createDiscordThreadPayload.thread_id.toString(),
      "GetDiscordThread - GET /discord/threads/:thread_id - response body should have correct markdown content":
        (r) =>
          r.message.discordThread.markdownContent ===
          createDiscordThreadPayload.markdown_content,
      "GetDiscordThread - GET /discord/threads/:thread_id - response body should have correct url":
        (r) =>
          r.message.discordThread.url.url === createDiscordThreadPayload.url,
      "GetDiscordThread - GET /discord/threads/:thread_id - response body should have correct created_timestamp_at_discord":
        (r) =>
          Date.parse(r.message.discordThread.createdTimestampAtDiscord) /
            1000 ===
          createDiscordThreadPayload.created_timestamp_at_discord,
      "GetDiscordThread - GET /discord/threads/:thread_id - response body should have created_timestamp_at_curioucity":
        (r) =>
          typeof r.message.discordThread.createdTimestampAtCurioucity !==
            "undefined" &&
          r.message.discordThread.created_timestamp_at_curioucity !== null,
    });

    const deleteDiscordThreadResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordThread",
      {
        thread_id: createDiscordThreadPayload.thread_id,
      }
    );

    check(deleteDiscordThreadResponse, {
      "GetDiscordThread - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const listDiscordThread = () => {
  group("gRPC DiscordService - Should list discord thread", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const testSize = 10;
    const newDiscordThreads = [];

    for (let i = 0; i < testSize; i++) {
      const createDiscordThreadPayload = {
        thread_id: `${Math.floor(Math.random() * 100000000)}`,
        markdown_content: "Hi i am here",
        url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
        created_timestamp_at_discord: 1675220675,
      };
      newDiscordThreads.push(createDiscordThreadPayload);
    }

    const ListNotExistDiscordThreadsResponse = client.invoke(
      "third_party.v1alpha.DiscordService/ListDiscordThread",
      {}
    );

    check(ListNotExistDiscordThreadsResponse, {
      "ListDiscordThread - no discord thread exist, should response StatusOK": (
        r
      ) => r.status === grpc.StatusOK,
      "ListDiscordThread - no discord thread exist, should response empty data":
        (r) => r.message.discordThreads.length === 0,
    });

    for (const discordThread of newDiscordThreads) {
      const createDiscordThreadResponse = client.invoke(
        "third_party.v1alpha.DiscordService/CreateDiscordThread",
        discordThread
      );

      check(createDiscordThreadResponse, {
        "ListDiscordThread - create test discord thread - response status should be StatusOK":
          (r) => r.status === grpc.StatusOK,
      });
    }

    const ListExistDiscordThreadsResponse = client.invoke(
      "third_party.v1alpha.DiscordService/ListDiscordThread",
      {}
    );

    check(ListExistDiscordThreadsResponse, {
      "ListDiscordThread - test discord guilds exist, should response StatusOK":
        (r) => r.status === grpc.StatusOK,
      "ListDiscordThread - test discord guilds exist, should have correct data":
        (r) => r.message.discordThreads.length === testSize,
    });

    for (const discordThread of newDiscordThreads) {
      const deleteDiscordThreadResponse = client.invoke(
        "third_party.v1alpha.DiscordService/DeleteDiscordThread",
        {
          thread_id: discordThread.thread_id,
        }
      );

      check(deleteDiscordThreadResponse, {
        "ListDiscordThread - delete discord thread - response status should be StatusOK":
          (r) => r.status === grpc.StatusOK,
      });
    }
  });
};
