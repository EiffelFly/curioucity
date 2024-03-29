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
      "GetDiscordGuild - delete tested discord guild - response status should be StatusOK":
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
        "CleanUp - clean up all discord guilds": (r) =>
          r.status === grpc.StatusOK,
      });
    }
  }
};
