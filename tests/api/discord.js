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
  group("Discord - Should delete disocrd message", () => {
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
  group("Discord - Should list discord guilds", () => {
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

export const createDiscordMessage = () => {
  group("Disocrd - Should create discord message", () => {
    const discordMessageId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordMessagePayload = {
      message_id: discordMessageId,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/messages/create`,
        JSON.stringify(createDiscordMessagePayload),
        {
          headers,
        }
      ),
      {
        "createDiscordMessage - POST /discord/messages/create - response status should be 201":
          (r) => r.status === 201,
        "createDiscordMessage - POST /discord/messages/create - response body should have id":
          (r) =>
            typeof r.json().discord_message.id !== undefined &&
            r.json().discord_message.id !== null,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct message_id":
          (r) =>
            r.json().discord_message.message_id ===
            createDiscordMessagePayload.message_id.toString(),
        "createDiscordMessage - POST /discord/messages/create - response body should have correct content":
          (r) =>
            r.json().discord_message.content ===
            createDiscordMessagePayload.content,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct markdown content":
          (r) =>
            r.json().discord_message.markdown_content ===
            createDiscordMessagePayload.markdown_content,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct url":
          (r) =>
            r.json().discord_message.url.url ===
            createDiscordMessagePayload.url,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_message.created_timestamp_at_discord) /
              1000 ===
            createDiscordMessagePayload.created_timestamp_at_discord,
        "createDiscordMessage - POST /discord/messages/create - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_message.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_message.created_timestamp_at_curioucity !== null,
        "createDiscordMessage - POST /discord/messages/create - response body should have correct order_in_thread":
          (r) =>
            r.json().discord_message.order_in_thread ===
            createDiscordMessagePayload.order_in_thread,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/messages/${discordMessageId}`,
        undefined,
        { headers }
      ),
      {
        "createDiscordMessage - DELETE /discord/messages/:message_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const deleteDiscordMessage = () => {
  group("Discord - Should delete disocrd message", () => {
    const messageId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordMessagePayload = {
      message_id: messageId,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/messages/create`,
        JSON.stringify(createDiscordMessagePayload),
        {
          headers,
        }
      ),
      {
        "deleteDiscordMessage - POST /discord/messages/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/messages/${messageId}`,
        undefined,
        { headers }
      ),
      {
        "deleteDiscordMessage - DELETE /discord/messages/:message_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const getDiscordMessage = () => {
  group("Discord - Should get discord message", () => {
    // Should return not found when try to get not exist discord message
    const notExistMessageId = `${Math.floor(Math.random() * 100000000)}`;

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/messages/${notExistMessageId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordMessage - GET /discord/messages/:message_id - not exist discord message, response status should be 404":
          (r) => r.status === 404,
      }
    );

    // Should get the newly created discord message

    const newMessageId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordMessagePayload = {
      message_id: newMessageId,
      content: "Hi i am here",
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
      order_in_thread: 20,
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/messages/create`,
        JSON.stringify(createDiscordMessagePayload),
        {
          headers,
        }
      ),
      {
        "getDiscordMessage - POST /discord/messages/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/messages/${newMessageId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordMessage - GET /discord/messages/:message_id - response status should be 200":
          (r) => r.status === 200,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have id":
          (r) =>
            typeof r.json().discord_message.id !== undefined &&
            r.json().discord_message.id !== null,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct message_id":
          (r) =>
            r.json().discord_message.message_id ===
            createDiscordMessagePayload.message_id.toString(),
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct content":
          (r) =>
            r.json().discord_message.content ===
            createDiscordMessagePayload.content,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct markdown content":
          (r) =>
            r.json().discord_message.markdown_content ===
            createDiscordMessagePayload.markdown_content,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct url":
          (r) =>
            r.json().discord_message.url.url ===
            createDiscordMessagePayload.url,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_message.created_timestamp_at_discord) /
              1000 ===
            createDiscordMessagePayload.created_timestamp_at_discord,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_message.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_message.created_timestamp_at_curioucity !== null,
        "getDiscordMessage - GET /discord/messages/:message_id - response body should have correct order_in_thread":
          (r) =>
            r.json().discord_message.order_in_thread ===
            createDiscordMessagePayload.order_in_thread,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/messages/${newMessageId}`,
        undefined,
        { headers }
      ),
      {
        "deleteDiscordMessage - DELETE /discord/messages/:message_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const listDiscordMessage = () => {
  group("Discord - Should list discord messages", () => {
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

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request("GET", `${API_HOST}/discord/messages`, undefined, {
        headers,
      }),
      {
        "listDiscordMessage - GET /discord/messages - should response 200": (
          r
        ) => r.status === 200,
        // In the future, we have to stop protobuf from emit default value
        "listDiscordMessage - GET /discord/messages - no discord message exist":
          (r) => r.json().discord_messages === undefined,
      }
    );

    for (const discordMessage of newDiscordMessages) {
      check(
        http.request(
          "POST",
          `${API_HOST}/discord/messages/create`,
          JSON.stringify(discordMessage),
          {
            headers,
          }
        ),
        {
          "listDiscordMessage - POST /discord/messages/create - response status should be 201":
            (r) => r.status === 201,
        }
      );
    }

    check(
      http.request("GET", `${API_HOST}/discord/messages`, undefined, {
        headers,
      }),
      {
        "listDiscordMessage - GET /discord/messages - should response 200": (
          r
        ) => r.status === 200,
        "listDiscordMessage - GET /discord/messages - should have 10 discord messages":
          (r) => r.json().discord_messages.length === 10,
      }
    );

    for (const discordMessage of newDiscordMessages) {
      check(
        http.request(
          "DELETE",
          `${API_HOST}/discord/messages/${discordMessage.message_id}`,
          undefined,
          {
            headers,
          }
        ),
        {
          "listDiscordMessage - DELETE /discord/messages/:message_id - response status should be 204":
            (r) => r.status === 204,
        }
      );
    }
  });
};

export const createDiscordThread = () => {
  group("Disocrd - Should create discord thread", () => {
    const discordThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordThreadPayload = {
      thread_id: discordThreadId,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/threads/create`,
        JSON.stringify(createDiscordThreadPayload),
        {
          headers,
        }
      ),
      {
        "createDiscordThread - POST /discord/threads/create - response status should be 201":
          (r) => r.status === 201,
        "createDiscordThread - POST /discord/threads/create - response body should have id":
          (r) =>
            typeof r.json().discord_thread.id !== undefined &&
            r.json().discord_thread.id !== null,
        "createDiscordThread - POST /discord/threads/create - response body should have correct thread_id":
          (r) =>
            r.json().discord_thread.thread_id ===
            createDiscordThreadPayload.thread_id.toString(),
        "createDiscordThread - POST /discord/threads/create - response body should have correct markdown content":
          (r) =>
            r.json().discord_thread.markdown_content ===
            createDiscordThreadPayload.markdown_content,
        "createDiscordThread - POST /discord/threads/create - response body should have correct url":
          (r) =>
            r.json().discord_thread.url.url === createDiscordThreadPayload.url,
        "createDiscordThread - POST /discord/threads/create - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_thread.created_timestamp_at_discord) /
              1000 ===
            createDiscordThreadPayload.created_timestamp_at_discord,
        "createDiscordThread - POST /discord/threads/create - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_thread.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_thread.created_timestamp_at_curioucity !== null,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/threads/${discordThreadId}`,
        undefined,
        { headers }
      ),
      {
        "createDiscordThread - POST /discord/threads/:thread_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const deleteDiscordThread = () => {
  group("Discord - Should delete disocrd message", () => {
    const discordThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordThreadPayload = {
      thread_id: discordThreadId,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/threads/create`,
        JSON.stringify(createDiscordThreadPayload),
        {
          headers,
        }
      ),
      {
        "deleteDiscordThread - POST /discord/threads/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/threads/${discordThreadId}`,
        undefined,
        { headers }
      ),
      {
        "deleteDiscordThread - POST /discord/threads/:thread_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const getDiscordThread = () => {
  group("Discord - Should get discord thread", () => {
    // Should return not found when try to get not exist discord message
    const notExistThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/threads/${notExistThreadId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordThread - GET /discord/threads/:thread_id - not exist discord thead, response status should be 404":
          (r) => r.status === 404,
      }
    );

    // Should get the newly created discord message

    const newThreadId = `${Math.floor(Math.random() * 100000000)}`;

    const createDiscordThreadPayload = {
      thread_id: newThreadId,
      markdown_content: "Hi i am here",
      url: `https://discord.com/id/${Math.floor(Math.random() * 100000000)}`,
      created_timestamp_at_discord: 1675220675,
    };

    check(
      http.request(
        "POST",
        `${API_HOST}/discord/threads/create`,
        JSON.stringify(createDiscordThreadPayload),
        {
          headers,
        }
      ),
      {
        "getDiscordThread - POST /discord/threads/create - response status should be 201":
          (r) => r.status === 201,
      }
    );

    check(
      http.request(
        "GET",
        `${API_HOST}/discord/threads/${newThreadId}`,
        undefined,
        {
          headers,
        }
      ),
      {
        "getDiscordThread - GET /discord/threads/:thread_id - response status should be 200":
          (r) => r.status === 200,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have id":
          (r) =>
            typeof r.json().discord_thread.id !== undefined &&
            r.json().discord_thread.id !== null,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct thread_id":
          (r) =>
            r.json().discord_thread.thread_id ===
            createDiscordThreadPayload.thread_id.toString(),
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct markdown content":
          (r) =>
            r.json().discord_thread.markdown_content ===
            createDiscordThreadPayload.markdown_content,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct url":
          (r) =>
            r.json().discord_thread.url.url === createDiscordThreadPayload.url,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have correct created_timestamp_at_discord":
          (r) =>
            Date.parse(r.json().discord_thread.created_timestamp_at_discord) /
              1000 ===
            createDiscordThreadPayload.created_timestamp_at_discord,
        "getDiscordThread - GET /discord/threads/:thread_id - response body should have created_timestamp_at_curioucity":
          (r) =>
            typeof r.json().discord_thread.created_timestamp_at_curioucity !==
              "undefined" &&
            r.json().discord_thread.created_timestamp_at_curioucity !== null,
      }
    );

    check(
      http.request(
        "DELETE",
        `${API_HOST}/discord/threads/${newThreadId}`,
        undefined,
        { headers }
      ),
      {
        "getDiscordThread - DELETE /discord/threads/:thread_id - response status should be 204":
          (r) => r.status === 204,
      }
    );
  });
};

export const listDiscordThread = () => {
  group("Discord - Should list discord threads", () => {
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

    const headers = {
      "Content-Type": "application/json",
    };

    check(
      http.request("GET", `${API_HOST}/discord/messages`, undefined, {
        headers,
      }),
      {
        "listDiscordThread - GET /discord/threads - should response 200": (r) =>
          r.status === 200,
        // In the future, we have to stop protobuf from emit default value
        "listDiscordThread - GET /discord/threads - no discord thread exist": (
          r
        ) => r.json().discord_messages === undefined,
      }
    );

    for (const discordThread of newDiscordThreads) {
      check(
        http.request(
          "POST",
          `${API_HOST}/discord/threads/create`,
          JSON.stringify(discordThread),
          {
            headers,
          }
        ),
        {
          "listDiscordThread - POST /discord/threads/create - response status should be 201":
            (r) => r.status === 201,
        }
      );
    }

    check(
      http.request("GET", `${API_HOST}/discord/threads`, undefined, {
        headers,
      }),
      {
        "listDiscordThread - GET /discord/threads - should response 200": (r) =>
          r.status === 200,
        "listDiscordThread - GET /discord/threads - should have 10 discord messages":
          (r) => r.json().discord_threads.length === 10,
      }
    );

    for (const discordThread of newDiscordThreads) {
      check(
        http.request(
          "DELETE",
          `${API_HOST}/discord/threads/${discordThread.thread_id}`,
          undefined,
          {
            headers,
          }
        ),
        {
          "listDiscordThread - DELETE /discord/threads/:thread_id - response status should be 204":
            (r) => r.status === 204,
        }
      );
    }
  });
};
