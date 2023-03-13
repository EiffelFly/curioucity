import grpc from "k6/net/grpc";
import { check, group } from "k6";
import { GRPC_API_HOST } from "./grpc.js";

const client = new grpc.Client();

client.load(
  ["../../packages/curioucity/proto"],
  "third_party/v1alpha/discord_service.proto"
);

export const createDiscordGuild = () => {
  group("gRPC DiscordService - Should create discord guild", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createDiscordGuildPayload = {
      guild_id: `${Math.floor(Math.random() * 100000000)}`,
      icon: "icon",
      name: "Curioucity",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const createDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordGuild",
      createDiscordGuildPayload
    );

    check(createDiscordGuildResponse, {
      "CreateDiscordGuild - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "CreateDiscordGuild - response body should have id": (r) =>
        typeof r.message.discordGuild.id !== undefined &&
        r.message.discordGuild.id !== null,
      "CreateDiscordGuild - response body should have correct guild_id": (r) =>
        r.message.discordGuild.guildId ===
        createDiscordGuildPayload.guild_id.toString(),
      "CreateDiscordGuild - response body should have correct icon": (r) =>
        r.message.discordGuild.icon === createDiscordGuildPayload.icon,
      "CreateDiscordGuild - response body should have correct name": (r) =>
        r.message.discordGuild.name === createDiscordGuildPayload.name,
      "CreateDiscordGuild - response body should have correct url": (r) =>
        r.message.discordGuild.url.url === createDiscordGuildPayload.url,
      "CreateDiscordGuild - response body should have correct created_timestamp_at_discord":
        (r) =>
          Date.parse(r.message.discordGuild.createdTimestampAtDiscord) /
            1000 ===
          createDiscordGuildPayload.created_timestamp_at_discord,
      "CreateDiscordGuild - response body should have created_timestamp_at_curioucity":
        (r) =>
          typeof r.message.discordGuild.createdTimestampAtDiscord !==
            "undefined" &&
          r.message.discordGuild.created_timestamp_at_curioucity !== null,
    });

    const deleteDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordGuild",
      {
        guild_id: createDiscordGuildPayload.guild_id,
      }
    );

    check(deleteDiscordGuildResponse, {
      "CreateDiscordGuild - delete test discord guild - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });
  });
};

export const deleteDiscordGuild = () => {
  group("gRPC DiscordService - Should delete disocrd guild", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const createDiscordGuildPayload = {
      guild_id: `${Math.floor(Math.random() * 100000000)}`,
      icon: "icon",
      name: "Curioucity",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const createDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordGuild",
      createDiscordGuildPayload
    );

    check(createDiscordGuildResponse, {
      "DeleteDiscordGuild - create test discord guild - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });

    const deleteDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordGuild",
      {
        guild_id: createDiscordGuildPayload.guild_id,
      }
    );

    check(deleteDiscordGuildResponse, {
      "DeleteDiscordGuild - delete discord guild - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });
  });
};

export const getDiscordGuild = () => {
  group("gRPC DiscordService - Should get discord guild", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });

    const getNotExistDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/GetDiscordGuild",
      {
        guild_id: `${Math.floor(Math.random() * 100000000)}`,
      }
    );

    check(getNotExistDiscordGuildResponse, {
      "GetDiscordGuild - get not exist discord guild, response status should be StatusNotFound":
        (r) => r.status === grpc.StatusNotFound,
    });

    const createDiscordGuildPayload = {
      guild_id: `${Math.floor(Math.random() * 100000000)}`,
      icon: "icon",
      name: "Curioucity",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const createDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/CreateDiscordGuild",
      createDiscordGuildPayload
    );

    check(createDiscordGuildResponse, {
      "GetDiscordGuild - create test discord guild - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });

    const getExistDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/GetDiscordGuild",
      {
        guild_id: createDiscordGuildPayload.guild_id,
      }
    );

    check(getExistDiscordGuildResponse, {
      "GetDiscordGuild  - response status should be StatusOK": (r) =>
        r.status === grpc.StatusOK,
      "GetDiscordGuild - response body should have id": (r) =>
        typeof r.message.discordGuild.id !== undefined &&
        r.message.discordGuild.id !== null,
      "GetDiscordGuild - response body should have correct guild_id": (r) =>
        r.message.discordGuild.guildId ===
        createDiscordGuildPayload.guild_id.toString(),
      "GetDiscordGuild - response body should have correct icon": (r) =>
        r.message.discordGuild.icon === createDiscordGuildPayload.icon,
      "GetDiscordGuild - response body should have correct name": (r) =>
        r.message.discordGuild.name === createDiscordGuildPayload.name,
      "GetDiscordGuild - response body should have correct url": (r) =>
        r.message.discordGuild.url.url === createDiscordGuildPayload.url,
      "GetDiscordGuild - response body should have correct created_timestamp_at_discord":
        (r) =>
          Date.parse(r.message.discordGuild.createdTimestampAtDiscord) /
            1000 ===
          createDiscordGuildPayload.created_timestamp_at_discord,
      "GetDiscordGuild - response body should have created_timestamp_at_curioucity":
        (r) =>
          typeof r.message.discordGuild.createdTimestampAtDiscord !==
            "undefined" &&
          r.message.discordGuild.createdTimestampAtDiscord !== null,
    });

    const deleteDiscordGuildResponse = client.invoke(
      "third_party.v1alpha.DiscordService/DeleteDiscordGuild",
      {
        guild_id: createDiscordGuildPayload.guild_id,
      }
    );

    check(deleteDiscordGuildResponse, {
      "DeleteDiscordGuild - delete discord guild - response status should be StatusOK":
        (r) => r.status === grpc.StatusOK,
    });
  });
};

export const listDiscordGuild = () => {
  group("gRPC DiscordService - Should list discord guild", () => {
    client.connect(GRPC_API_HOST, { timeout: 2000, plaintext: true });
    const testSize = 10;
    const newDiscordGuilds = [];

    for (let i = 0; i < testSize; i++) {
      const createDiscordGuildPayload = {
        guild_id: `${Math.floor(Math.random() * 100000000)}`,
        icon: "icon",
        name: "Curioucity",
        url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
        created_timestamp_at_discord: 1675220675,
      };
      newDiscordGuilds.push(createDiscordGuildPayload);
    }

    const listNotExistDiscordGuildsResponse = client.invoke(
      "third_party.v1alpha.DiscordService/ListDiscordGuild",
      {}
    );

    check(listNotExistDiscordGuildsResponse, {
      "ListDiscordGuild - no discord guild exist, should response StatusOK": (
        r
      ) => r.status === grpc.StatusOK,
      "ListDiscordGuild - no discord guild exist, should response urls is empty":
        (r) => r.message.discordGuilds.length === 0,
    });

    for (const discordGuild of newDiscordGuilds) {
      const createDiscordGuildResponse = client.invoke(
        "third_party.v1alpha.DiscordService/CreateDiscordGuild",
        discordGuild
      );

      check(createDiscordGuildResponse, {
        "ListDiscordGuild - create test discord guild - response status should be StatusOK":
          (r) => r.status === grpc.StatusOK,
      });
    }

    const listExistDiscordGuildsResponse = client.invoke(
      "third_party.v1alpha.DiscordService/ListDiscordGuild",
      {}
    );

    check(listExistDiscordGuildsResponse, {
      "ListDiscordGuild - test discord guilds exist, should response StatusOK":
        (r) => r.status === grpc.StatusOK,
      "ListDiscordGuild - test discord guilds exis, should response the correct data":
        (r) => r.message.discordGuilds.length === testSize,
    });

    for (const discordGuild of newDiscordGuilds) {
      const deleteDiscordGuildResponse = client.invoke(
        "third_party.v1alpha.DiscordService/DeleteDiscordGuild",
        {
          guild_id: discordGuild.guild_id,
        }
      );

      check(deleteDiscordGuildResponse, {
        "DeleteDiscordGuild - delete discord guild - response status should be StatusOK":
          (r) => r.status === grpc.StatusOK,
      });
    }
  });
};

export const cleanupDiscordGuild = () => {
  const listDiscordGuildResponse = client.invoke(
    "third_party.v1alpha.DiscordService/ListDiscordGuild",
    {}
  );

  if (
    listDiscordGuildResponse.status === grpc.StatusOK &&
    listDiscordGuildResponse.message.discordGuilds &&
    listDiscordGuildResponse.message.discordGuilds.length !== 0
  ) {
    for (const discordGuild of listDiscordGuildResponse.message.discordGuilds) {
      const deleteDiscordGuildResponse = client.invoke(
        "third_party.v1alpha.DiscordService/DeleteDiscordGuild",
        {
          guild_id: discordGuild.guild_id,
        }
      );

      check(deleteDiscordGuildResponse, {
        "CleanUp - clean up all discord guild": (r) =>
          r.status === grpc.StatusOK,
      });
    }
  }
};

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
