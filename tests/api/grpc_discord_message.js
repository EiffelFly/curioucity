import grpc from "k6/net/grpc";
import { check, group } from "k6";
import { GRPC_API_HOST } from "./grpc.js";

const client = new grpc.Client();

client.load(
  ["../../packages/curioucity/proto"],
  "third_party/v1alpha/discord_service.proto"
);

export const createDiscordMessage = () => {
  group("gRPC DiscordService - Should create discord message", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createDiscordMessagePayload = {
      message_id: `${Math.floor(Math.random() * 100000000)}`,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    const createDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordMessage",
      createDiscordMessagePayload
    );

    check(createDiscordMessageResponse, {
      "CreateDiscordMessage - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "CreateDiscordMessage - response body should have id": (r) =>
        typeof r.message.discordMessage.id !== undefined &&
        r.message.discordMessage.id !== null,
      "CreateDiscordMessage - response body should have correct message_id": (
        r
      ) =>
        r.message.discordMessage.messageId ===
        createDiscordMessagePayload.message_id.toString(),
      "CreateDiscordMessage - response body should have correct content": (r) =>
        r.message.discordMessage.content ===
        createDiscordMessagePayload.content,
      "CreateDiscordMessage - response body should have correct markdown content":
        (r) =>
          r.message.discordMessage.markdownContent ===
          createDiscordMessagePayload.markdown_content,
      "CreateDiscordMessage - response body should have correct url": (r) =>
        r.message.discordMessage.url.url === createDiscordMessagePayload.url,
      "CreateDiscordMessage - response body should have correct created_timestamp_at_discord":
        (r) =>
          Date.parse(r.message.discordMessage.createdTimestampAtDiscord) /
            1000 ===
          createDiscordMessagePayload.created_timestamp_at_discord,
      "CreateDiscordMessage - response body should have created_timestamp_at_curioucity":
        (r) =>
          typeof r.message.discordMessage.createdTimestampAtCurioucity !==
            "undefined" &&
          r.message.discordMessage.createdTimestampAtCurioucity !== null,
      "CreateDiscordMessage - response body should have correct order_in_thread":
        (r) =>
          r.message.discordMessage.orderInThread ===
          createDiscordMessagePayload.order_in_thread,
    });

    const deleteDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordMessage",
      {
        message_id: createDiscordMessagePayload.message_id,
      }
    );

    check(deleteDiscordMessageResponse, {
      "CreateDiscordMessage - delete test discord message - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });
  });
};

export const deleteDiscordMessage = () => {
  group("gRPC DiscordService - Should delete disocrd message", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createDiscordMessagePayload = {
      message_id: `${Math.floor(Math.random() * 100000000)}`,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    const createDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordMessage",
      createDiscordMessagePayload
    );

    check(createDiscordMessageResponse, {
      "DeleteDiscordMessage - create test discord message - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });

    const deleteDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordMessage",
      {
        message_id: createDiscordMessagePayload.message_id,
      }
    );

    check(deleteDiscordMessageResponse, {
      "DeleteDiscordMessage - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const getDiscordMessage = () => {
  group("gRPC DiscordService - Should get discord thread", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const getNotExistDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/GetDiscordMessage",
      {
        message_id: `${Math.floor(Math.random() * 100000000)}`,
      }
    );

    check(getNotExistDiscordMessageResponse, {
      "GetDiscordMessage - get not exist discord message, response status should be StatusNotFound":
        (r) => r.status === grpc.StatusNotFound,
    });

    const createDiscordMessagePayload = {
      message_id: `${Math.floor(Math.random() * 100000000)}`,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    const createDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordMessage",
      createDiscordMessagePayload
    );

    check(createDiscordMessageResponse, {
      "GetDiscordMessage - create test discord message - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });

    const getExistDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/GetDiscordMessage",
      {
        message_id: createDiscordMessagePayload.message_id,
      }
    );

    check(getExistDiscordMessageResponse, {
      "GetDiscordMessage - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "GetDiscordMessage - response body should have id": (r) =>
        typeof r.message.discordMessage.id !== undefined &&
        r.message.discordMessage.id !== null,
      "GetDiscordMessage - response body should have correct message_id": (r) =>
        r.message.discordMessage.messageId ===
        createDiscordMessagePayload.message_id.toString(),
      "GetDiscordMessage - response body should have correct content": (r) =>
        r.message.discordMessage.content ===
        createDiscordMessagePayload.content,
      "GetDiscordMessage - response body should have correct markdown content":
        (r) =>
          r.message.discordMessage.markdownContent ===
          createDiscordMessagePayload.markdown_content,
      "GetDiscordMessage - response body should have correct url": (r) =>
        r.message.discordMessage.url.url === createDiscordMessagePayload.url,
      "GetDiscordMessage - response body should have correct created_timestamp_at_discord":
        (r) =>
          Date.parse(r.message.discordMessage.createdTimestampAtDiscord) /
            1000 ===
          createDiscordMessagePayload.created_timestamp_at_discord,
      "GetDiscordMessage - response body should have created_timestamp_at_curioucity":
        (r) =>
          typeof r.message.discordMessage.createdTimestampAtCurioucity !==
            "undefined" &&
          r.message.discordMessage.created_timestamp_at_curioucity !== null,
      "GetDiscordMessage - response body should have correct order_in_thread": (
        r
      ) =>
        r.message.discordMessage.orderInThread ===
        createDiscordMessagePayload.order_in_thread,
    });

    const deleteDiscordMessageResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordMessage",
      {
        message_id: createDiscordMessagePayload.message_id,
      }
    );

    check(deleteDiscordMessageResponse, {
      "GetDiscordMessage - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
    });
  });
};

export const listDiscordMessage = () => {
  group("gRPC DiscordService - Should list discord message", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const testSize = 10;
    const newDiscordMessages = [];

    for (let i = 0; i < testSize; i++) {
      const createDiscordMessagePayload = {
        message_id: `${Math.floor(Math.random() * 100000000)}`,
        content: "Hi i am here",
        markdown_content: "Hi i am here",
        url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
        created_timestamp_at_discord: 1675220675,
        order_in_thread: 20,
      };
      newDiscordMessages.push(createDiscordMessagePayload);
    }

    const ListNotExistDiscordMessagesResponse = client.invoke(
      "third_party.v1alpha.DiscordService/ListDiscordMessage",
      {}
    );

    check(ListNotExistDiscordMessagesResponse, {
      "ListDiscordMessage - no discord message exist, should response StatusOK":
        (r) => r.status === grpc.StatusOK,
      "ListDiscordMessage - no discord message exist, should response empty data":
        (r) => r.message.discordMessages.length === 0,
    });

    for (const discordMessage of newDiscordMessages) {
      const createDiscordMessageResponse = client.invoke(
        "third_party.v1alpha.DiscordService/CreateDiscordMessage",
        discordMessage
      );

      check(createDiscordMessageResponse, {
        "ListDiscordMessage - create test discord message - response status should be StatusOK":
          (r) => r.status === grpc.StatusOK,
      });
    }

    const ListExistDiscordMessagesResponse = client.invoke(
      "third_party.v1alpha.DiscordService/ListDiscordMessage",
      {}
    );

    check(ListExistDiscordMessagesResponse, {
      "ListDiscordMessage - test discord messsages exist, should response StatusOK":
        (r) => r.status === grpc.StatusOK,
      "ListDiscordMessage - test discord messsages exist, should have correct data":
        (r) => r.message.discordMessages.length === testSize,
    });

    for (const discordMessage of newDiscordMessages) {
      const deleteDiscordMessageResponse = client.invoke(
        "third_party.v1alpha.DiscordService/DeleteDiscordMessage",
        {
          message_id: discordMessage.message_id,
        }
      );

      check(deleteDiscordMessageResponse, {
        "ListDiscordMessage - delete discord message - response status should be StatusOK":
          (r) => r.status === grpc.StatusOK,
      });
    }
  });
};

export const cleanupDiscordMessage = () => {
  const ListDiscordMessageResponse = client.invoke(
    "third_party.v1alpha.DiscordService/ListDiscordMessage",
    {}
  );

  if (
    ListDiscordMessageResponse.status === grpc.StatusOK &&
    ListDiscordMessageResponse.message.discordMessages &&
    ListDiscordMessageResponse.message.discordMessages.length !== 0
  ) {
    for (const discordMessage of ListDiscordMessageResponse.message
      .discordMessages) {
      const deleteDiscordMessageResponse = client.invoke(
        "third_party.v1alpha.DiscordService/DeleteDiscordMessage",
        {
          message_id: discordMessage.message_id,
        }
      );

      check(deleteDiscordMessageResponse, {
        "CleanUp - clean up all discord messages": (r) =>
          r.status === grpc.StatusOK,
      });
    }
  }
};
