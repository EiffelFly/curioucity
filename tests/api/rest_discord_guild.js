import http from "k6/http";
import { check, group } from "k6";
import { API_HOST } from "./rest.js";

export const createDiscordGuild = () => {
  group("Disocrd - Should create discord guild", () => {
    const createDiscordGuildPayload = {
      guild_id: `${Math.floor(Math.random() * 100000000)}`,
      icon: "icon",
      name: "Curioucity",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/guilds/create`,
        JSON.stringify(createDiscordGuildPayload),
        {
          headers,
        }
      ),
      {
        "createDiscordGuild - POST /discord/guilds/create - response status should be 201":
          (r) => r.status === 201,
        "createDiscordGuild - POST /discord/guilds/create - response body should have id":
          (r) =>
            typeof r.json().discord_guild.id !== undefined &&
            r.json().discord_guild.id !== null,
        "createDiscordGuild - POST /discord/guilds/create - response body should have correct message_id":
          (r) =>
            r.json().discord_guild.guild_id ===
            createDiscordGuildPayload.guild_id.toString(),
        "createDiscordGuild - POST /discord/guilds/create - response body should have correct icon":
          (r) => r.json().discord_guild.icon === createDiscordGuildPayload.icon,
        "createDiscordGuild - POST /discord/guilds/create - response body should have correct name":
          (r) => r.json().discord_guild.name === createDiscordGuildPayload.name,
        "createDiscordGuild - POST /discord/guilds/create - response body should have correct url":
          (r) =>
            r.json().discord_guild.url.url === createDiscordGuildPayload.url,
        "createDiscordGuild - POST /discord/guilds/create - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_guild.created_timestamp_at_discord) /
              1000 ===
            createDiscordGuildPayload.created_timestamp_at_discord,
        "createDiscordGuild - POST /discord/guilds/create - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_guild.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_guild.created_timestamp_at_curioucity !== null,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/guilds/${createDiscordGuildPayload.guild_id}`,
        undefined,
        { headers }
      ),
      {
        "createDiscordGuild - DELETE /discord/guilds/:guild_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const deleteDiscordGuild = () => {
  group("Discord - Should delete disocrd guild", () => {
    const guildId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordGuildPayload = {
      guild_id: guildId,
      icon: "icon",
      name: "Curioucity",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/guilds/create`,
        JSON.stringify(createDiscordGuildPayload),
        {
          headers,
        }
      ),
      {
        "deleteDiscordGuild - POST /discord/guilds/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/guilds/${guildId}`,
        undefined,
        { headers }
      ),
      {
        "deleteDiscordGuild - DELETE /discord/guilds/:guild_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const getDiscordGuild = () => {
  group("Discord - Should get discord guild", () => {
    const notExistGuildId = `${Math.floor(Math.random() * 100000000)}`;

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/guilds/${notExistGuildId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordGuild - GET /discord/guilds/:guild_id - not exist discord guild, response status should be 404":
          (r) => r.status === 404,
      }
    );

    const newGuildId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordGuildPayload = {
      guild_id: newGuildId,
      icon: "icon",
      name: "Curioucity",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/guilds/create`,
        JSON.stringify(createDiscordGuildPayload),
        {
          headers,
        }
      ),
      {
        "getDiscordGuild - POST /discord/guilds/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/guilds/${newGuildId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordGuild - GET /discord/guilds/:guild_id - response status should be 200":
          (r) => r.status === 200,
        "getDiscordGuild - GET /discord/guilds/:guild_id - response body should have id":
          (r) =>
            typeof r.json().discord_guild.id !== undefined &&
            r.json().discord_guild.id !== null,
        "getDiscordGuild - GET /discord/guilds/:guild_id - response body should have correct guild_id":
          (r) =>
            r.json().discord_guild.guild_id ===
            createDiscordGuildPayload.guild_id.toString(),
        "getDiscordGuild - GET /discord/guilds/:guild_id - response body should have correct icon":
          (r) => r.json().discord_guild.icon === createDiscordGuildPayload.icon,
        "getDiscordGuild - GET /discord/guilds/:guild_id - response body should have correct name":
          (r) => r.json().discord_guild.name === createDiscordGuildPayload.name,
        "getDiscordGuild - GET /discord/guilds/:guild_id - response body should have correct url":
          (r) =>
            r.json().discord_guild.url.url === createDiscordGuildPayload.url,
        "getDiscordGuild - GET /discord/guilds/:guild_id - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_guild.created_timestamp_at_discord) /
              1000 ===
            createDiscordGuildPayload.created_timestamp_at_discord,
        "getDiscordGuild - GET /discord/guilds/:guild_id - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_guild.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_guild.created_timestamp_at_curioucity !== null,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/guilds/${newGuildId}`,
        undefined,
        { headers }
      ),
      {
        "getDiscordGuild - DELETE /discord/guilds/:guild_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const listDiscordGuild = () => {
  group("Discord - Should list discord guild", () => {
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

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request("GET", `${API_HOST}/discord/guilds`, undefined, {
        headers,
      }),
      {
        "listDiscordGuild - GET /discord/guilds - should response 200": (r) =>
          r.status === 200,
        // In the future, we have to stop protobuf from emit default value
        "listDiscordGuild - GET /discord/guilds - no discord guild exist": (
          r
        ) => r.json().discord_guilds === undefined,
      }
    );

    for (const discordGuild of newDiscordGuilds) {
      check(
        http.request(
          "POST",
          `${API_HOST}/discord/guilds/create`,
          JSON.stringify(discordGuild),
          {
            headers,
          }
        ),
        {
          "listDiscordGuild - POST /discord/guilds/create - response status should be 201":
            (r) => r.status === 201,
        }
      );
    }

    check(
      http.request("GET", `${API_HOST}/discord/guilds`, undefined, {
        headers,
      }),
      {
        "listDiscordGuild - GET /discord/guilds - should response 200": (r) =>
          r.status === 200,
        "listDiscordGuild - GET /discord/guilds - should have 10 discord guild":
          (r) => r.json().discord_guilds.length === 10,
      }
    );

    for (const discordGuild of newDiscordGuilds) {
      check(
        http.request(
          "DELETE",
          `${API_HOST}/discord/guilds/${discordGuild.guild_id}`,
          undefined,
          {
            headers,
          }
        ),
        {
          "listDiscordGuild - DELETE /discord/guilds/:guild_id - response status should be 204":
            (r) => r.status === 204,
        }
      );
    }
  });
};
